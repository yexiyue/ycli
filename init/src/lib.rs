use clap::{arg, Args};
use command::interface::CommandAction;
use utils::{
    make,
    prelude::{debug, error, info, trace, warn},
    spinner::Spinner,
};
mod template_list;
use dirs;
use fs_extra::{copy_items, dir::get_dir_content};
use reqwest;
use serde_json::{self, Value};
use std::fs::{self, remove_dir_all};
use std::process::Command;
use template_list::ADD_TEMPLATE;

#[derive(Debug, Args)]
pub struct InitArgs {
    ///项目名称
    name: Option<String>,

    ///当目录不为空时强制安装
    #[arg(short, long, default_value = "false")]
    force: bool,

    ///模板(template-vue3/template-react18)
    #[arg(short, long)]
    template: Option<String>,
}

impl CommandAction for InitArgs {
    fn action(&mut self) {
        //通过命令行传参或者交互获取名称和npm_name
        self.ensure_name_template();

        //获取模板最新版本
        let version = self.get_template_version().unwrap();
        //下载模板
        self.download_template(version);

        //把缓存目录中的模板拷贝到当前工作目录
        self.cope_template()
    }
}

impl InitArgs {
    fn ensure_name_template(&mut self) {
        //如果没有项目名称就让用户进行输入
        if self.name.is_none() {
            self.name = Some(make::make_input("请输入项目名称\n"));
        }
        //打印输入信息
        trace!("{:?}", self.name);
        let list: Vec<String> = ADD_TEMPLATE
            .iter()
            .map(|x| x.value.clone().to_string())
            .collect();

        if self.template.is_none() {
            let index = make::make_select("请选择模板", &list);
            //打印选择的索引
            trace!("index:{}", index);

            //根据索引获取模板对应的npm_name
            self.template = Some(ADD_TEMPLATE[index].npm_name.clone().to_string());
        } else if !ADD_TEMPLATE //对name进行判断是否在ADD_TEMPLATE列表中，如果不在则提示用户并让用户进行选择
            .iter()
            .any(|item| -> bool { item.value == self.template.as_ref().unwrap() })
        {
            trace!("{:?}", self.template);
            warn!("template not found");
            let index = make::make_select("请重新选择模板", &list);
            //根据索引获取模板对应的npm_name
            self.template = Some(ADD_TEMPLATE[index].npm_name.clone().to_string());
        } else {
            let index = ADD_TEMPLATE
                .iter()
                .find(|x| x.value == self.template.as_ref().unwrap())
                .unwrap();

            self.template = Some(index.npm_name.clone().to_string())
        }

        debug!("{:?}", self.template);
    }

    fn get_template_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        //拼接地址
        let url = format!(
            "https://registry.npmjs.org/{}",
            self.template.clone().unwrap()
        );
        //请求npm数据
        let res: Value = reqwest::blocking::get(url)?.json()?;
        let version = res.get("dist-tags").unwrap().get("latest").unwrap();
        debug!("{:?}", version);
        Ok(version.to_string())
    }

    //把模板安装到用户目录缓存下
    fn download_template(&self, version: String) {
        //开启spinner
        let spinner = Spinner::new("下载模板中...".to_string());
        spinner.start();

        //获取用户目录
        let home_dir = dirs::home_dir().unwrap();
        let node_modules_dir = home_dir.join(".ycli").join("node_modules");
        //创建用户缓存目录
        fs::create_dir_all(node_modules_dir).unwrap();

        //获取npm执行路径
        let npm_path =
            String::from_utf8(Command::new("where").arg("npm").output().unwrap().stdout).unwrap();

        let npm_path_vec: Vec<String> = npm_path
            .split("\n")
            .filter(|x| x.len() > 0)
            .map(|x| x.replace("\r", ""))
            .collect();
        debug!("{:?}", npm_path_vec);

        let mut result;

        for path in npm_path_vec {
            //对每个路径进行执行
            result = Command::new(path)
                .current_dir(home_dir.join(".ycli"))
                .arg("install")
                .arg(format!(
                    "{}@{}",
                    self.template.clone().unwrap(),
                    version.replace("\"", "")
                ))
                .output();

            //一旦有一个成功就退出
            if result.is_ok() {
                spinner.stop(None);
                info!("模板下载成功");
                break;
            }
        }
    }

    //拷贝目录到当前工作目录
    fn cope_template(&self) {
        //获取模板路径
        let template_dir = dirs::home_dir()
            .unwrap()
            .join(".ycli")
            .join("node_modules")
            .join(self.template.clone().unwrap())
            .join("template");

        trace!("{:?}", template_dir);

        //拼接目标路径
        let target_dir = std::env::current_dir()
            .unwrap()
            .join(self.name.clone().unwrap());
        trace!("{:?}", target_dir);

        //获取目录下所有文件
        let files = fs::read_dir(template_dir).unwrap();

        //保存到数组中
        let mut templates = vec![];

        for file in files {
            let file = file.unwrap();
            templates.push(file.path());
        }

        let count = get_dir_content(target_dir.clone());

        if count.is_ok() {
            if count.unwrap().files.len() > 0 {
                if self.force {
                    warn!("目录不为空强制安装");
                    remove_dir_all(target_dir.clone()).unwrap();
                    fs::create_dir_all(target_dir.clone()).unwrap();
                } else {
                    error!("安装失败,安装目录不为空,可以使用-f,--force强制安装");
                }
            }
        } else {
            //先创建目标目录
            fs::create_dir_all(target_dir.clone()).unwrap();
        }
        let options = fs_extra::dir::CopyOptions::new();

        trace!("{:?}", templates);
        copy_items(&templates, target_dir.clone(), &options).unwrap();
        info!("安装成功");
        info!("进入项目目录cd {}", self.name.clone().unwrap());
    }
}
