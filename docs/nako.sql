-- 导出 nako_blog 的数据库结构
CREATE DATABASE IF NOT EXISTS `nako_blog` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;
USE `nako_blog`;

-- 导出  表 nako_blog.nako_art 结构
DROP TABLE IF EXISTS `nako_art`;
CREATE TABLE IF NOT EXISTS `nako_art` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `uuid` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT 'id',
  `cate_id` int unsigned NOT NULL DEFAULT '0' COMMENT '分类ID',
  `user_id` int unsigned NOT NULL DEFAULT '0' COMMENT '作者',
  `cover` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '封面',
  `title` varchar(150) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '标题',
  `keywords` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '关键字',
  `description` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '描述',
  `content` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '内容',
  `tags` varchar(250) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '标签',
  `from` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '来源',
  `views` bigint unsigned DEFAULT '1' COMMENT '阅读量',
  `is_top` tinyint unsigned DEFAULT '0' COMMENT '1-置顶',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='文章';

-- 正在导出表  nako_blog.nako_art 的数据：0 rows
/*!40000 ALTER TABLE `nako_art` DISABLE KEYS */;
/*!40000 ALTER TABLE `nako_art` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_attach 结构
DROP TABLE IF EXISTS `nako_attach`;
CREATE TABLE IF NOT EXISTS `nako_attach` (
  `id` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `name` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '文件名',
  `path` varchar(250) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '文件路径',
  `ext` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '文件类型',
  `size` int NOT NULL DEFAULT '0' COMMENT '文件大小',
  `md5` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '文件md5',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci ROW_FORMAT=COMPACT COMMENT='附件表';

-- 正在导出表  nako_blog.nako_attach 的数据：~1 rows (大约)
/*!40000 ALTER TABLE `nako_attach` DISABLE KEYS */;
/*!40000 ALTER TABLE `nako_attach` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_cate 结构
DROP TABLE IF EXISTS `nako_cate`;
CREATE TABLE IF NOT EXISTS `nako_cate` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `pid` int NOT NULL DEFAULT '0' COMMENT '父级ID',
  `name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '名称',
  `slug` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '标志',
  `desc` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '描述',
  `sort` int DEFAULT '100' COMMENT '排序',
  `list_tpl` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '列表模板',
  `view_tpl` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '详情模板',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=13 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='分类';

-- 正在导出表  nako_blog.nako_cate 的数据：3 rows
/*!40000 ALTER TABLE `nako_cate` DISABLE KEYS */;
REPLACE INTO `nako_cate` (`id`, `pid`, `name`, `slug`, `desc`, `sort`, `list_tpl`, `view_tpl`, `status`, `add_time`, `add_ip`) VALUES
	(1, 0, '热门八卦', 'rmbg', '热门八卦热门八卦', 100, 'cate', 'view', 1, 1655823356, ''),
	(6, 0, '影视综合', 'video', '影视综合', 100, 'cate', 'view', 1, 1655823356, '127.0.0.1'),
	(9, 0, '诗词江湖', 'scjh', '诗词江湖111', 95, 'cate', 'view', 1, 1655823356, '127.0.0.1');
/*!40000 ALTER TABLE `nako_cate` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_comment 结构
DROP TABLE IF EXISTS `nako_comment`;
CREATE TABLE IF NOT EXISTS `nako_comment` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `art_id` int NOT NULL DEFAULT '0' COMMENT '文件ID',
  `user_id` int NOT NULL DEFAULT '0' COMMENT '账号',
  `reply_id` int DEFAULT NULL COMMENT '回复ID',
  `content` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '评论内容',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='评论';

-- 正在导出表  nako_blog.nako_comment 的数据：0 rows
/*!40000 ALTER TABLE `nako_comment` DISABLE KEYS */;
/*!40000 ALTER TABLE `nako_comment` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_page 结构
DROP TABLE IF EXISTS `nako_page`;
CREATE TABLE IF NOT EXISTS `nako_page` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `user_id` int unsigned NOT NULL DEFAULT '0' COMMENT '作者',
  `slug` varchar(150) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '标志',
  `title` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '标题',
  `keywords` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT '' COMMENT '关键字',
  `description` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '描述',
  `content` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '内容',
  `tpl` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '模板',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci ROW_FORMAT=DYNAMIC COMMENT='单页';

-- 正在导出表  nako_blog.nako_page 的数据：1 rows
/*!40000 ALTER TABLE `nako_page` DISABLE KEYS */;
REPLACE INTO `nako_page` (`id`, `user_id`, `slug`, `title`, `keywords`, `description`, `content`, `tpl`, `status`, `add_time`, `add_ip`) VALUES
	(1, 1, 'aboutme', '关于我们', '', '', '<p>关于我们</p><p><img src="/upload/images/24a13a9a5aaa6270d7cc567d5958c23c.jpg" alt="/upload/images/24a13a9a5aaa6270d7cc567d5958c23c.jpg" data-href="/upload/images/24a13a9a5aaa6270d7cc567d5958c23c.jpg" style=""/></p>', 'page-about', 1, 1655823356, '127.0.0.1');
/*!40000 ALTER TABLE `nako_page` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_setting 结构
DROP TABLE IF EXISTS `nako_setting`;
CREATE TABLE IF NOT EXISTS `nako_setting` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `key` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '字段',
  `value` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci COMMENT '字段值',
  `desc` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '字段说明',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='配置';

-- 正在导出表  nako_blog.nako_setting 的数据：6 rows
/*!40000 ALTER TABLE `nako_setting` DISABLE KEYS */;
REPLACE INTO `nako_setting` (`id`, `key`, `value`, `desc`) VALUES
	(1, 'website_name', '热门八卦王', '网站名称'),
	(2, 'website_keywords', '热门八卦王', '网站关键字'),
	(3, 'website_description', '热门八卦王', '网站描述'),
	(4, 'website_copyright', '版权1', '版权'),
	(5, 'website_status', '1', '网站关闭状态'),
	(6, 'website_beian', '网站备案2', '网站备案');
/*!40000 ALTER TABLE `nako_setting` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_tag 结构
DROP TABLE IF EXISTS `nako_tag`;
CREATE TABLE IF NOT EXISTS `nako_tag` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '名称',
  `desc` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '描述',
  `sort` int DEFAULT '100' COMMENT '排序',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='标签';

-- 正在导出表  nako_blog.nako_tag 的数据：2 rows
/*!40000 ALTER TABLE `nako_tag` DISABLE KEYS */;
REPLACE INTO `nako_tag` (`id`, `name`, `desc`, `sort`, `status`, `add_time`, `add_ip`) VALUES
	(1, '八卦', '八卦八卦', 101, 1, 1655823356, '127.0.0.1'),
	(2, '安卓', '安卓', 100, 1, 1655911967, '127.0.0.1');
/*!40000 ALTER TABLE `nako_tag` ENABLE KEYS */;

-- 导出  表 nako_blog.nako_user 结构
DROP TABLE IF EXISTS `nako_user`;
CREATE TABLE IF NOT EXISTS `nako_user` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `username` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '账号，大小写字母数字',
  `password` char(62) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '密码',
  `nickname` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '昵称',
  `avatar` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '头像',
  `sign` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '签名',
  `status` tinyint(1) DEFAULT '1' COMMENT '1-启用，0-禁用',
  `add_time` int NOT NULL DEFAULT '0' COMMENT '添加时间',
  `add_ip` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '添加IP',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户';

-- 正在导出表  nako_blog.nako_user 的数据：1 rows
REPLACE INTO `nako_user` (`id`, `username`, `password`, `nickname`, `avatar`, `sign`, `status`, `add_time`, `add_ip`) VALUES
	(1, 'username123', '', 'nickname123', NULL, 'signsign', NULL, 0, '');
