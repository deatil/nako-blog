use serde_json::json;
use serde_json::Value;

// 菜单
pub fn menus() -> Value {
    json!([
        {
            "id": 0,
            "title": "控制台",
            "icon": "layui-icon layui-icon-console",
            "type": 1,
            "openType": "_iframe",
            "href": "/admin/index/console"
        },
        {
            "id": "art",
            "title": "文章管理",
            "icon": "layui-icon layui-icon-app",
            "type": 0,
            "href": "",
            "children": [
                {
                    "id": 201,
                    "title": "文章列表",
                    "icon": "layui-icon layui-icon-face-smile",
                    "type": 1,
                    "openType": "_iframe",
                    "href": "/admin/art/index"
                },
                {
                    "id": 202,
                    "title": "分类管理",
                    "icon": "layui-icon layui-icon-file",
                    "type": 1,
                    "openType": "_iframe",
                    "href": "/admin/cate/index"
                },
                {
                    "id": 203,
                    "title": "评论管理",
                    "icon": "layui-icon layui-icon-file",
                    "type": 1,
                    "openType": "_iframe",
                    "href": "/admin/comment/index"
                },
                {
                    "id": 204,
                    "title": "标签管理",
                    "icon": "layui-icon layui-icon-file",
                    "type": 1,
                    "openType": "_iframe",
                    "href": "/admin/tag/index"
                }
            ]
        },
        {
            "id": "page",
            "title": "页面管理",
            "icon": "layui-icon layui-icon-star",
            "type": 1,
            "openType": "_iframe",
            "href": "/admin/page/index"
        },
        {
            "id": "guestbook",
            "title": "留言管理",
            "icon": "icon pear-icon pear-icon-complete",
            "type": 1,
            "openType": "_iframe",
            "href": "/admin/guestbook/index"
        },
        {
            "id": "system",
            "title": "系统管理",
            "icon": "layui-icon layui-icon-set-fill",
            "type": 0,
            "href": "",
            "children": [
                {
                    "id": 101,
                    "title": "用户管理",
                    "icon": "layui-icon layui-icon-user",
                    "type": 1,
                    "openType": "_iframe",
                    "href": "/admin/user/index"
                },
                {
                    "id": 102,
                    "title": "附件管理",
                    "icon": "layui-icon layui-icon-file",
                    "type": 1,
                    "openType": "_iframe",
                    "href": "/admin/attach/index"
                }
            ]
        },
        {
            "id": "setting",
            "title": "网站设置",
            "icon": "layui-icon layui-icon-auz",
            "type": 1,
            "openType": "_iframe",
            "href": "/admin/setting/index"
        }
    ])
}
