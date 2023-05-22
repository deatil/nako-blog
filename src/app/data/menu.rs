use serde_json::json;
use serde_json::Value;

// 菜单
pub fn menus() -> Value {
    json!([{
        "id": 0,
        "title": "控制台",
        "icon": "layui-icon layui-icon-console",
        "type": 1,
        "openType": "_iframe",
        "href": "/admin/index/console"
    },
    {
        "id": "system",
        "title": "系统管理",
        "icon": "layui-icon layui-icon-set-fill",
        "type": 0,
        "href": "",
        "children": [
            {
                "id": 2,
                "title": "用户管理",
                "icon": "layui-icon layui-icon-face-smile",
                "type": 1,
                "openType": "_iframe",
                "href": "/admin/user/index"
            }
        ]
    }])
}
