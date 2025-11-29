# OEIS TUI - 中文翻译

# Application
app-title = OEIS TUI
app-subtitle = 在线整数数列百科全书

# Greeting Screen
greeting-title = 欢迎使用 OEIS TUI
greeting-line1 = 一个美观的终端界面，用于探索整数数列
greeting-line2 = 按 'i' 或 '/' 开始搜索
greeting-line3 = 按 'r' 查看随机数列
greeting-line4 = 按 'w' 进入网络摄像头模式
greeting-line5 = 按 'Ctrl+H' 查看帮助
greeting-copyright = © OEIS Foundation Inc. - 所有数列数据均为 OEIS 财产
greeting-version = 版本 0.1.0

# 欢迎 / 空状态
welcome-title = 欢迎使用 OEIS TUI
welcome-subtitle = 在线整数数列百科全书 (非官方 TUI)
welcome-prompt = 输入一个数列、单词或 A 号开始。
welcome-search-label = 搜索 OEIS
welcome-enter-hint = Enter 搜索
welcome-esc-hint = Esc 关闭
welcome-hero-subtitle = 查找已知数列，发现参考文献并探索关系。
welcome-hero-tips = 示例：1,2,3,4,5,6  •  keyword:prime  •  id:A000045
welcome-hero-search-hint = 随时按 'i' 或 '/' 跳转到搜索。
search-empty-title = 暂无搜索结果
search-tips-title = 搜索技巧：
search-tip-terms = • 输入数列项：1,1,2,3,5,8,13
search-tip-anumber = • 按 A 编号搜索：id:A000045
search-tip-keyword = • 按关键词搜索：fibonacci
search-tip-prefixes = • 使用前缀：keyword:nice author:Sloane
search-start-hint = 按 'i' 或 '/' 开始搜索
search-recently-viewed = 最近查看
search-history-empty = 暂无历史记录
search-bookmarks-title = 书签
search-bookmarks-empty = 暂无书签。在详细视图中按 'b' 键可添加书签。
search-bookmarks-loading = 加载中...
search-bookmarks-notes = 笔记
search-results-title = 结果
# Search Screen
search-title = 搜索 OEIS
search-input-label = 搜索
search-input-placeholder = 输入数列项（例：1,2,3,5,8,13）、A编号或关键词...
search-status-results = { $count ->
    [0] 未找到结果
    *[other] 找到 { $count } 个结果
}
search-status-page = 第 { $current } 页，共 { $total } 页
search-status-loading = 搜索中...
search-status-error = 错误: { $message }
search-table-anumber = A编号
search-table-name = 名称
search-table-data = 数据预览
search-table-views = 浏览次数
search-help = i,/ 搜索 | ↑↓ 导航 | ←→ 翻页 | Enter 查看 | p 预览 | r 随机 | w 网络摄像头 | s 设定 | Ctrl+H 帮助 | q 退出

# Detail View
detail-tab-overview = 概述
detail-tab-formulas = 公式
detail-tab-code = 代码
detail-tab-references = 参考文献
detail-tab-crossrefs = 交叉引用
detail-tab-metadata = 元数据
detail-tab-graph = 图表
detail-tab-export = 导出
detail-offset = 偏移量
detail-keywords = 关键词
detail-author = 作者
detail-created = 创建时间
detail-modified = 最后修改
detail-comments = 评论
detail-data = 数列数据
detail-formulas = 公式
detail-examples = 示例
detail-maple = Maple 代码
detail-mathematica = Mathematica 代码
detail-programs = 其他程序
detail-references = 参考文献
detail-links = 链接
detail-crossrefs = 交叉引用
detail-extensions = 扩展
detail-no-data = 无可用数据
detail-help = Tab 切换 | ↑↓ 滚动 | g 图表 | e 导出 | o 浏览器 | b 书签 | Esc 返回
detail-help-next-link = 下一个链接
detail-help-prev-link = 上一个链接
detail-help-switch-tab = 切换标签
detail-help-follow-link = 打开链接
detail-help-scroll = 滚动
detail-help-graph = 图表
detail-help-export = 导出
detail-help-browser = 在浏览器中打开
detail-help-bookmark = 书签
detail-bookmarked = 已添加书签
detail-not-bookmarked = 未添加书签
detail-help-bfile = 获取B-file
detail-help-more = 更多
detail-help-modal-title = 详细视图 - 键盘快捷键
detail-bfile-available = 扩展数据可用
detail-bfile-fetch = 按 'f' 键获取B-file
detail-bfile-loading = 正在加载B-file...
detail-bfile-loaded = ✓ 已加载 {$count} 项
detail-bfile-error = B-file不可用
detail-bfile-not-found = 未找到此数列的B-file

# Graph View
graph-title = 图表视图
graph-line = 折线图
graph-scatter = 散点图
graph-log = 对数散点图
graph-pin = 针状图
graph-no-data = 没有可绘制的数值数据
graph-no-positive = 对数刻度没有正值
graph-current = 当前
graph-help = 1 折线 | 2 散点 | 3 对数 | 4 针状 | Esc 返回

# Export Screen
export-title = 导出数列
export-format = 选择格式
export-json = JSON
export-json-desc = 包含所有元数据的完整数列数据
export-csv = CSV
export-csv-desc = 逗号分隔格式的数列值
export-txt = TXT
export-txt-desc = 人类可读的纯文本格式
export-markdown = Markdown
export-markdown-desc = 格式化文档
export-preview = 预览
export-no-sequence = 没有要导出的数列
export-success = 已成功导出到剪贴板
export-file-success = 已保存到文件: { $path }
export-error = 导出失败: { $message }
export-help = ↑↓ 选择 | 1-5 快速选择 | Enter 剪贴板 | Ctrl+S 保存 | Esc 取消
export-bfile = B-file
export-bfile-desc = 扩展数列数据（索引 值 对）
export-bfile-not-loaded = B-file未加载 - 在详细视图中按'f'
export-select-format = 选择格式
export-cancel = 取消

# 导出内容标签
export-label-offset = 偏移量
export-label-keywords = 关键词
export-label-data = 数据
export-label-author = 作者
export-label-created = 创建时间
export-label-modified = 最后修改
export-label-references = 参考文献
export-label-revision = 修订版本

# 导出章节标题
export-section-sequence-data = 数列数据
export-section-metadata = 元数据
export-section-comments = 评论
export-section-formulas = 公式
export-section-examples = 示例
export-section-code = 代码
export-section-references = 参考文献
export-section-links = 链接
export-section-crossrefs = 交叉引用

# 导出子章节标题
export-subsection-maple = Maple
export-subsection-mathematica = Mathematica
export-subsection-programs = 其他程序

# 导出格式特定
export-csv-header = A编号,名称,值
export-markdown-source = 来源
export-markdown-oeis-credit = 来自在线整数数列百科全书(OEIS)的数据

# Webcam Mode
webcam-title = OEIS 网络摄像头 - 数列浏览器
webcam-category = 类别
webcam-category-all = 所有数列
webcam-category-all-desc = 浏览所有 OEIS 数列
webcam-category-best = 最佳数列
webcam-category-best-desc = 有趣且值得注意的数列（关键词:nice）
webcam-category-needing = 需要项
webcam-category-needing-desc = 请求更多项的数列（关键词:more）
webcam-category-recent = 最近添加
webcam-category-recent-desc = 最近添加的数列（关键词:new）
webcam-interval = 刷新间隔
webcam-interval-manual = 手动
webcam-interval-manual-desc = 按空格键前进
webcam-interval-5s = 5秒
webcam-interval-5s-desc = 每5秒自动刷新
webcam-interval-10s = 10秒
webcam-interval-10s-desc = 每10秒自动刷新
webcam-interval-20s = 20秒
webcam-interval-20s-desc = 每20秒自动刷新
webcam-interval-30s = 30秒
webcam-interval-30s-desc = 每30秒自动刷新
webcam-interval-1m = 1分钟
webcam-interval-1m-desc = 每60秒自动刷新
webcam-current-sequence = 当前数列
webcam-no-sequence = 未加载数列
webcam-load-first = 按空格键或回车键加载第一个数列
webcam-refresh-in = 下次刷新倒计时 { $seconds } 秒...
webcam-more-comments = ... 以及另外 { $count } 条评论
webcam-help = Space/Enter 下一个 | ←→ 类别 | ↑↓ 间隔 | 0-5 快速 | d 详情 | Esc 返回

# Settings Screen
settings-title = 设置
settings-language = 语言
settings-language-desc = 选择界面语言
settings-theme = 主题
settings-theme-desc = 配色方案（即将推出）
settings-cache = 缓存
settings-cache-desc = 管理本地缓存
settings-cache-clear = 清除缓存
settings-cache-size = 缓存大小: { $size }
settings-help = ↑↓ 导航 | Enter 选择 | Esc 返回

# About Screen
about-title = 关于 OEIS TUI
about-version = 版本
about-author = 创建者
about-license = 许可证
about-built-with = 技术栈
about-links = 链接
about-repository = 代码仓库
about-oeis-link = OEIS 网站
about-disclaimer = 这是一个非官方客户端，未经 The OEIS Foundation Inc. 附属或认可。

# Help Screen
help-title = 帮助 - 键盘快捷键
help-global = 全局控制
help-global-quit = 退出应用程序
help-global-help = 显示/隐藏帮助
help-global-back = 返回 / 取消
help-search = 搜索屏幕
help-search-input = 开始搜索
help-search-navigate = 导航结果
help-search-page = 上一页/下一页
help-search-view = 查看选定的数列
help-search-random = 随机数列
help-search-preview = 切换预览面板
help-search-preview-tabs = 切换预览标签
help-search-mouse-select = 点击选择结果
help-search-mouse-open = 双击打开结果
help-search-mouse-scroll = 滚轮滚动预览/结果
help-search-webcam = 网络摄像头模式
help-detail = 详细视图
help-detail-links = 循环高亮链接
help-detail-tabs = 切换标签
help-detail-open-link = 打开高亮链接
help-detail-scroll = 滚动内容
help-detail-scroll-fast = 快速滚动
help-detail-graph = 查看图表
help-detail-export = 导出数列
help-detail-browser = 在浏览器中打开
help-detail-bookmark = 切换书签
help-graph = 图表视图
help-graph-types = 切换图表类型
help-export = 导出屏幕
help-export-select = 选择格式
help-export-quick = 快速格式选择
help-export-clipboard = 导出到剪贴板
help-export-file = 保存到文件
help-webcam = 网络摄像头模式
help-webcam-next = 加载下一个数列
help-webcam-category = 切换类别
help-webcam-interval = 更改刷新间隔
help-webcam-quick = 快速间隔选择
help-webcam-detail = 跳转到详细视图

# Common
common-loading = 加载中...
common-error = 错误
common-success = 成功
common-cancel = 取消
common-ok = 确定
common-yes = 是
common-no = 否
common-back = 返回
common-next = 下一个
common-previous = 上一个
common-page = 页
common-of = 共

# Errors
error-network = 网络错误: 无法连接到 OEIS
error-api = API 错误: { $message }
error-parse = 解析错误: 无效的数据格式
error-cache = 缓存错误: { $message }
error-clipboard = 剪贴板错误: { $message }
error-file = 文件错误: { $message }
error-unknown = 发生未知错误
search-status-fetching = 正在从 OEIS 获取结果，请稍候
search-no-results = 未找到结果
search-result-one = 找到1个结果
search-result-many = 找到{ $count }个结果
search-result-many-plus = 找到{ $count }+个结果
search-block-results = 结果
search-block-preview = 预览
search-block-details = 详情
search-preview-empty = 无预览
search-invalid-tab = 无效的选项卡
search-view-count = { $count ->
    [one] 1次浏览
    *[other] { $count }次浏览
}
search-help-search = 搜索
search-help-navigate = 导航
search-help-page = 页面
search-help-view = 查看
search-help-preview = 预览
search-help-bookmarks = 书签
search-help-random = 随机
search-help-webcam = 网络摄像头
search-help-settings = 设置
search-help-help = 帮助
search-help-quit = 退出
search-help-click = 选择
search-help-click-x2 = 打开
search-help-scroll = 移动
detail-no-sequence = 未加载序列
detail-block-sequence = 序列
detail-block-details = 详情
detail-section-data = 数据
detail-section-comments = 评论
detail-section-examples = 示例
graph-help-line = 折线
graph-help-scatter = 散点
graph-help-log = 对数
graph-help-pin = 针图
graph-help-back = 返回详细视图
webcam-sequence-offset = 偏移量
webcam-sequence-keywords = 关键词
webcam-sequence-data-title = 序列数据
webcam-sequence-comments-title = 评论
webcam-help-next = 下一个
webcam-help-category = 类别
webcam-help-interval = 间隔
webcam-help-quick = 快速选择
webcam-help-detail = 详细视图
webcam-help-back = 返回
settings-block-themes = 主题
settings-block-animation = 欢迎动画
settings-help-switch = 切换部分
settings-help-navigate = 导航
settings-help-apply = 应用
settings-help-cycle-theme = 循环主题
settings-help-back = 返回
