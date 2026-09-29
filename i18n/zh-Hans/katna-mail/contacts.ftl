# Katna Mail, Chinese (Simplified) (简体中文): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = 联系人
contacts-frequent = 常用联系人
contacts-other = 其他联系人
contacts-other-about = 您发过邮件但未保存的 Gmail 联系人
contacts-other-email = 发送电子邮件
contacts-other-empty = 没有其他联系人。您通过 Gmail 发过邮件但未保存的人会显示在这里。
contacts-other-allow = 要查看其他联系人，请重新登录您的 Gmail 账号，并允许 Katna 查看它们。
contacts-labels = 标签
contacts-label-options = 标签选项
contacts-label-rename = 重命名标签
contacts-label-email = 给所有人发邮件
contacts-label-delete = 删除标签
contacts-label-new = 新建标签
contacts-label-name = 标签名称
contacts-label-button = 标签
contacts-label-menu = 标签为：
contacts-label-added = 已添加到“{ $name }”
contacts-label-removed = 已从“{ $name }”中移除
contacts-label-renamed = 标签已重命名为“{ $name }”
contacts-label-deleted = 已删除标签“{ $name }”
contacts-label-no-email = 此标签下没有人有电子邮件地址
contacts-manage = 修复和管理
contacts-merge = 合并和修复
contacts-merge-about = { $count ->
   *[other] { $count } 条建议：看起来是同一个人的联系人
}
contacts-merge-none = 没有重复项。姓名或电话号码相同的联系人会显示在这里。
contacts-merge-count = { $count ->
   *[other] { $count } 位联系人
}
contacts-merge-all = 全部合并
contacts-merge-button = 合并
contacts-merge-dismiss = 忽略
contacts-merged = { $count ->
    [1] 已合并联系人
   *[other] 已完成 { $count } 次合并
}
contacts-import = 导入
contacts-export = 导出
contacts-import-file = 从 vCard 或 CSV 文件导入联系人
contacts-imported = { $count ->
   *[other] 已将 { $count } 位联系人导入到“{ $place }”
}
contacts-imported-some = { $count ->
   *[other] 已将 { $count } 位联系人导入到“{ $place }”；已保存的 { $skipped } 位已略过
}
contacts-import-none = 在 { $name } 中未找到联系人
contacts-import-all-saved = { $name } 中的所有人都已保存
contacts-import-failed = 无法读取 { $name }：{ $error }
contacts-exported = { $count ->
   *[other] 已将 { $count } 位联系人导出到 { $path }
}
contacts-export-none = 没有可导出的联系人
contacts-export-failed = 无法导出联系人：{ $error }
contacts-print = 打印
contacts-print-title = 联系人
contacts-print-none = 没有可打印的联系人
contacts-print-typed = { $value }（{ $kind }）
contacts-print-birthday = 生日：{ $day }
contacts-print-nickname = 昵称：{ $name }
contacts-create = 创建联系人

## Search and the list

contacts-search = 搜索联系人
contacts-loading = 正在加载联系人…
contacts-empty = 尚无已保存的联系人。您在 Gmail、Outlook 或邮件服务中保存的联系人会显示在这里。
contacts-empty-no-books = 账号中的联系人同步完成后会显示在这里。
contacts-none-found = 没有与搜索匹配的联系人。
contacts-starred = { $count ->
   *[other] 已加星标的联系人 ({ $count })
}
contacts-count = 联系人 ({ $count })
contacts-col-name = 姓名
contacts-col-email = 电子邮件
contacts-col-phone = 电话号码
contacts-col-job = 职位和公司
contacts-col-labels = 标签

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = 允许 Katna 读取 { $address } 的联系人。
contacts-allow-many = { $more ->
   *[other] 允许 Katna 读取 { $address } 及另外 { $more } 个账号的联系人。
}
contacts-allow-button = 允许

## A contact's page

contacts-back = 返回联系人
contacts-edit = 修改
contacts-delete = 删除
contacts-qr = 通过二维码分享
contacts-qr-about = 用手机摄像头扫描即可保存该联系人。
contacts-qr-too-long = 此联系人的详细信息过多，无法放入二维码。
contacts-qr-done = 完成
contacts-deleted = 已删除 { $name }
contacts-added = 已将 { $name } 添加到联系人
contacts-find-mail = 邮件
contacts-details = 联系人详情
contacts-saved-in = 保存位置
contacts-notes = 笔记
contacts-birthday = 生日
contacts-nickname = 昵称
contacts-this-computer = 此计算机
contacts-kind-home = 住宅
contacts-kind-work = 工作
contacts-kind-mobile = 手机
contacts-kind-other = 其他
contacts-source-google = Google 通讯录
contacts-source-microsoft = Outlook 联系人
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = 创建联系人
contacts-edit-title = 修改联系人
contacts-edit-save = 保存
contacts-edit-saving = 正在保存…
contacts-edit-cancel = 取消
contacts-saved = 联系人已保存
contacts-edit-save-to = 保存到
contacts-edit-changes-go-to = 更改将保存到 { $place }。
contacts-edit-given = 名字
contacts-edit-family = 姓氏
contacts-edit-company = 公司
contacts-edit-job = 职位
contacts-edit-email = 电子邮件
contacts-edit-phone = 电话
contacts-edit-with-kind = { $field }（{ $kind }）
contacts-edit-add-email = 添加电子邮件地址
contacts-edit-add-phone = 添加电话号码
contacts-edit-street = 街道地址
contacts-edit-city = 城市
contacts-edit-postcode = 邮政编码
contacts-edit-country = 国家/地区
contacts-edit-birthday = 生日（YYYY-MM-DD）
contacts-edit-empty = 请先添加姓名、电子邮件或电话号码。
