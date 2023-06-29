#[derive(Debug)]
pub struct Template {
    pub value: &'static str,
    pub npm_name: &'static str,
    pub version: &'static str,
}

//定义const数组类型时得这样定义
pub const ADD_TEMPLATE: [Template;2] = [
    Template {
        value: "template-vue3",
        npm_name: "@yexiyue.com/template-vue3",
        version: "1.0.0",
    },
    Template {
        value: "template-react18",
        npm_name: "@yexiyue.com/template-react18",
        version: "1.0.0",
    },
];

