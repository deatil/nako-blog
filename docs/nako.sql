-- 导出 nako_blog 的数据库结构
CREATE DATABASE IF NOT EXISTS `nako_blog` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;
USE `nako_blog`;

-- 导出  表 nako_blog.nako_art 结构
CREATE TABLE IF NOT EXISTS `nako_art` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `uuid` char(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT 'id',
  `cate_id` int unsigned NOT NULL DEFAULT '0' COMMENT '分类ID',
  `user_id` int unsigned NOT NULL DEFAULT '0' COMMENT '作者',
  `title` varchar(150) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '标题',
  `keywords` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT '' COMMENT '关键字',
  `description` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '描述',
  `cover` varchar(150) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '封面',
  `content` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '内容',
  `tags` varchar(250) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '标签',
  `from` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT '' COMMENT '来源',
  `views` bigint unsigned DEFAULT '0' COMMENT '阅读量',
  `is_top` tinyint(1) DEFAULT '0' COMMENT '1-置顶',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='文章';

-- 正在导出表  nako_blog.nako_art 的数据：0 rows
/*!40000 ALTER TABLE `nako_art` DISABLE KEYS */;
REPLACE INTO `nako_art` (`id`, `uuid`, `cate_id`, `user_id`, `title`, `keywords`, `description`, `cover`, `content`, `tags`, `from`, `views`, `is_top`, `status`, `add_time`, `add_ip`) VALUES
	(2, 'fd57a201-ed89-4d5c-882a-5f0d6a98f2c7', 1, 1, '123 123', '', '测试', '/upload/images/5517eb15-d516-43c8-8d1c-00d4fb6e685e.png', '<p>123123</p><p><img src="../../upload/images/fe48e124-8462-4bce-a7d0-be16cf0a140d.jpg" alt="" data-mce-src="../../upload/images/fe48e124-8462-4bce-a7d0-be16cf0a140d.jpg" width="1080" height="1080"></p>', '你好', '网络', 0, 0, 1, 1685274804, '127.0.0.1');
/*!40000 ALTER TABLE `nako_art` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_attach 结构
CREATE TABLE IF NOT EXISTS `nako_attach` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '文件名',
  `path` varchar(250) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '文件路径',
  `ext` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '文件类型',
  `size` int(10) unsigned zerofill NOT NULL DEFAULT '0000000000' COMMENT '文件大小',
  `md5` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '文件md5',
  `type` tinyint DEFAULT '0' COMMENT '附件类型。1-附件,2-图片',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci ROW_FORMAT=COMPACT COMMENT='附件表';

-- 正在导出表  nako_blog.nako_attach 的数据：2 rows
/*!40000 ALTER TABLE `nako_attach` DISABLE KEYS */;
REPLACE INTO `nako_attach` (`id`, `name`, `path`, `ext`, `size`, `md5`, `type`, `status`, `add_time`, `add_ip`) VALUES
	(1, 'avatar.jpg', '/images/fe48e124-8462-4bce-a7d0-be16cf0a140d.jpg', 'jpg', 0000076158, 'f1ae3f3797de2749bc8a9baab7523935', 2, 1, 1685158261, '127.0.0.1'),
	(2, 'show.png', '/images/5517eb15-d516-43c8-8d1c-00d4fb6e685e.png', 'png', 0000056400, '12ba50cf659d15124676f96494476e88', 2, 1, 1685161497, '127.0.0.1');
/*!40000 ALTER TABLE `nako_attach` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_cate 结构
CREATE TABLE IF NOT EXISTS `nako_cate` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `pid` int unsigned NOT NULL DEFAULT '0' COMMENT '父级ID',
  `name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '名称',
  `slug` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '标志',
  `desc` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '描述',
  `sort` int DEFAULT '100' COMMENT '排序',
  `list_tpl` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '列表模板',
  `view_tpl` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '详情模板',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=14 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='分类';

-- 正在导出表  nako_blog.nako_cate 的数据：3 rows
/*!40000 ALTER TABLE `nako_cate` DISABLE KEYS */;
REPLACE INTO `nako_cate` (`id`, `pid`, `name`, `slug`, `desc`, `sort`, `list_tpl`, `view_tpl`, `status`, `add_time`, `add_ip`) VALUES
	(1, 0, '热门八卦', 'rmbg', '热门八卦热门八卦', 100, 'list_a.html', 'view_a.html', 1, 1655823356, ''),
	(6, 0, '影视综合', 'video', '影视综合', 100, 'list_a.html', 'view_a.html', 1, 1655823356, '127.0.0.1'),
	(9, 0, '诗词江湖', 'scjh', '诗词江湖11', 95, 'list_a.html', 'view_a.html', 1, 1655823356, '127.0.0.1');
/*!40000 ALTER TABLE `nako_cate` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_comment 结构
CREATE TABLE IF NOT EXISTS `nako_comment` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `art_id` int unsigned NOT NULL DEFAULT '0' COMMENT '文件ID',
  `reply_id` int unsigned DEFAULT NULL COMMENT '回复ID',
  `username` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '账号',
  `email` varchar(150) COLLATE utf8mb4_unicode_ci DEFAULT '' COMMENT '邮箱',
  `content` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '评论内容',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='评论';

-- 正在导出表  nako_blog.nako_comment 的数据：0 rows
/*!40000 ALTER TABLE `nako_comment` DISABLE KEYS */;
/*!40000 ALTER TABLE `nako_comment` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_page 结构
CREATE TABLE IF NOT EXISTS `nako_page` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `user_id` int unsigned NOT NULL DEFAULT '0' COMMENT '作者',
  `slug` varchar(150) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '标志',
  `title` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '标题',
  `keywords` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT '' COMMENT '关键字',
  `description` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '描述',
  `content` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '内容',
  `tpl` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT '' COMMENT '模板',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci ROW_FORMAT=DYNAMIC COMMENT='单页';

-- 正在导出表  nako_blog.nako_page 的数据：1 rows
/*!40000 ALTER TABLE `nako_page` DISABLE KEYS */;
REPLACE INTO `nako_page` (`id`, `user_id`, `slug`, `title`, `keywords`, `description`, `content`, `tpl`, `status`, `add_time`, `add_ip`) VALUES
	(1, 1, 'aboutme', '关于我们', '111', '111', '<p>关于我们</p>', 'page_a.html', 1, 1655823356, '127.0.0.1'),
	(3, 1, 'liaixi', '联系我们', NULL, NULL, '', NULL, 1, 1685169754, '127.0.0.1');
/*!40000 ALTER TABLE `nako_page` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_setting 结构
CREATE TABLE IF NOT EXISTS `nako_setting` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `key` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '字段',
  `value` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci COMMENT '字段值',
  `desc` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT '' COMMENT '字段说明',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='配置';

-- 正在导出表  nako_blog.nako_setting 的数据：6 rows
/*!40000 ALTER TABLE `nako_setting` DISABLE KEYS */;
REPLACE INTO `nako_setting` (`id`, `key`, `value`, `desc`) VALUES
	(1, 'website_name', '热门八卦王', '名称'),
	(2, 'website_keywords', '热门八卦王', '关键字'),
	(3, 'website_description', '热门八卦王', '描述'),
	(4, 'website_copyright', '版权', '版权'),
	(5, 'website_status', '1', '状态'),
	(6, 'website_beian', '网站备案', '备案');
/*!40000 ALTER TABLE `nako_setting` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_tag 结构
CREATE TABLE IF NOT EXISTS `nako_tag` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '名称',
  `desc` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '描述',
  `sort` int DEFAULT '100' COMMENT '排序',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='标签';

-- 正在导出表  nako_blog.nako_tag 的数据：2 rows
/*!40000 ALTER TABLE `nako_tag` DISABLE KEYS */;
REPLACE INTO `nako_tag` (`id`, `name`, `desc`, `sort`, `status`, `add_time`, `add_ip`) VALUES
	(1, '八卦', '八卦八卦', 101, 1, 1655823356, '127.0.0.1'),
	(2, '安卓', '安卓', 100, 1, 1655911967, '127.0.0.1');
/*!40000 ALTER TABLE `nako_tag` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_user 结构
CREATE TABLE IF NOT EXISTS `nako_user` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `username` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '账号，大小写字母数字',
  `password` char(62) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '密码',
  `nickname` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '昵称',
  `avatar` varchar(150) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '头像',
  `sign` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '签名',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=15 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户';

-- 正在导出表  nako_blog.nako_user 的数据：2 rows
/*!40000 ALTER TABLE `nako_user` DISABLE KEYS */;
REPLACE INTO `nako_user` (`id`, `username`, `password`, `nickname`, `avatar`, `sign`, `status`, `add_time`, `add_ip`) VALUES
	(1, 'admin', '$2b$12$/aXiCMVd11/L5Mt0WonuiOfNLr81HJtNsIzLucYVVm9dNlZbcH7q.', '管理员', '/upload/avatar/356a192b7913b04c54574d18c28d46e6395428ab.jpg', 'signsign', 1, 1684299438, '127.0.0.1'),
	(10, 'test', '$2b$12$do.4tIJg4Q3Yd.A6Jwio9O9bqZrOQKvblvRShhFOWpanvGW5is1hy', 'test', NULL, 'test123123', 1, 1684304910, '127.0.0.1');
/*!40000 ALTER TABLE `nako_user` ENABLE KEYS */;

/*!40101 SET SQL_MODE=IFNULL(@OLD_SQL_MODE, '') */;
/*!40014 SET FOREIGN_KEY_CHECKS=IF(@OLD_FOREIGN_KEY_CHECKS IS NULL, 1, @OLD_FOREIGN_KEY_CHECKS) */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
