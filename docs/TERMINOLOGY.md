# 紫微斗数术语表

> 中文/拼音/英文对照表。

## 阴阳 (YinYang)

| 中文 | 拼音 | xstars 枚举 | 英文 |
|------|------|------|------|
| 阴 | Yin | `YinYang::Yin` | Yin |
| 阳 | Yang | `YinYang::Yang` | Yang |

## 五行 (Wuxing)

| 中文 | 拼音 | xstars 枚举 | 英文 | 相生 | 相克 |
|------|------|------|------|------|------|
| 木 | Mu | `Wuxing::Mu` | Wood | 木→火 | 木→土 |
| 火 | Huo | `Wuxing::Huo` | Fire | 火→土 | 火→金 |
| 土 | Tu | `Wuxing::Tu` | Earth | 土→金 | 土→水 |
| 金 | Jin | `Wuxing::Jin` | Metal | 金→水 | 金→木 |
| 水 | Shui | `Wuxing::Shui` | Water | 水→木 | 水→火 |

## 天干 (Tiangan)

| 中文 | 拼音 | xstars 枚举 | 代码 | 五行 | 阴阳 |
|------|------|------|------|------|------|
| 甲 | Jia | `Tiangan::Jia` | jia | 木 | 阳 |
| 乙 | Yi | `Tiangan::Yi` | yi | 木 | 阴 |
| 丙 | Bing | `Tiangan::Bing` | bing | 火 | 阳 |
| 丁 | Ding | `Tiangan::Ding` | ding | 火 | 阴 |
| 戊 | Wv | `Tiangan::Wv` | wv | 土 | 阳 |
| 己 | Ji | `Tiangan::Ji` | ji | 土 | 阴 |
| 庚 | Geng | `Tiangan::Geng` | geng | 金 | 阳 |
| 辛 | Xin | `Tiangan::Xin` | xin | 金 | 阴 |
| 壬 | Ren | `Tiangan::Ren` | ren | 水 | 阳 |
| 癸 | Gui | `Tiangan::Gui` | gui | 水 | 阴 |

> 代码中使用 `wv`（戊）而非 `wu`，避免与地支"午"冲突。

## 地支 (Dizhi)

| 中文 | 拼音 | xstars 枚举 | 英文 | 五行 | 生肖 | 地支索引 | 宫位坐标 |
|------|------|------|------|------|------|:--------:|:--------:|
| 子 | Zi | `Dizhi::Zi` | Zi | 水 | 鼠 | 0 | 10 |
| 丑 | Chou | `Dizhi::Chou` | Chou | 土 | 牛 | 1 | 11 |
| 寅 | Yin | `Dizhi::Yin` | Yin | 木 | 虎 | 2 | 0 |
| 卯 | Mao | `Dizhi::Mao` | Mao | 木 | 兔 | 3 | 1 |
| 辰 | Chen | `Dizhi::Chen` | Chen | 土 | 龙 | 4 | 2 |
| 巳 | Si | `Dizhi::Si` | Si | 火 | 蛇 | 5 | 3 |
| 午 | Wu | `Dizhi::Wu` | Wu | 火 | 马 | 6 | 4 |
| 未 | Wei | `Dizhi::Wei` | Wei | 土 | 羊 | 7 | 5 |
| 申 | Shen | `Dizhi::Shen` | Shen | 金 | 猴 | 8 | 6 |
| 酉 | You | `Dizhi::You` | You | 金 | 鸡 | 9 | 7 |
| 戌 | Xu | `Dizhi::Xu` | Xu | 土 | 狗 | 10 | 8 |
| 亥 | Hai | `Dizhi::Hai` | Hai | 水 | 猪 | 11 | 9 |

## 时辰 (Shichen)

| 中文 | 拼音 | xstars 枚举 | 时间范围 | 地支索引 |
|------|------|------|---------|:--------:|
| 子时 | zi | `Shichen::Zi` | 23:00-00:59 | 0 |
| 丑时 | chou | `Shichen::Chou` | 01:00-02:59 | 1 |
| 寅时 | yin | `Shichen::Yin` | 03:00-04:59 | 2 |
| 卯时 | mao | `Shichen::Mao` | 05:00-06:59 | 3 |
| 辰时 | chen | `Shichen::Chen` | 07:00-08:59 | 4 |
| 巳时 | si | `Shichen::Si` | 09:00-10:59 | 5 |
| 午时 | wu | `Shichen::Wu` | 11:00-12:59 | 6 |
| 未时 | wei | `Shichen::Wei` | 13:00-14:59 | 7 |
| 申时 | shen | `Shichen::Shen` | 15:00-16:59 | 8 |
| 酉时 | you | `Shichen::You` | 17:00-18:59 | 9 |
| 戌时 | xu | `Shichen::Xu` | 19:00-20:59 | 10 |
| 亥时 | hai | `Shichen::Hai` | 21:00-22:59 | 11 |

## 五行局 (WuxingGroup)

| 中文 | xstars 枚举 | 起运年龄 |
|------|------|:--------:|
| 水二局 | `WuxingGroup::Shui2` | 2 岁 |
| 木三局 | `WuxingGroup::Mu3` | 3 岁 |
| 金四局 | `WuxingGroup::Jin4` | 4 岁 |
| 土五局 | `WuxingGroup::Tu5` | 5 岁 |
| 火六局 | `WuxingGroup::Huo6` | 6 岁 |

## 星曜分类 (StarType)

| 中文 | 英文 | xstars 枚举 |
|------|------|------------|
| 主星 | Major | `StarType::Major` |
| 辅星 | Minor | `StarType::Minor` |
| 杂曜 | Misc | `StarType::Misc` |
| 神煞 | ShenSha | `StarType::ShenSha` |
| 运限星 | Yunxian | `StarType::Yunxian` |

### 神煞 48 颗 (ShenSha)

**长生十二神**：长生、沐浴、冠带、临官、帝旺、衰、病、死、墓、绝、胎、养

**博士十二神**：博士、力士、青龙、小耗、将军、奏书、飞廉、喜神、病符、大耗、伏兵、官府

**岁前十二神**：岁建、晦气、丧门、贯索、官符、小耗、大耗、龙德、白虎、天德、吊客、病符、岁破

**将前十二神**：将星、攀鞍、岁驿、息神、华盖、劫煞、灾煞、天煞、指背、咸池、月煞、亡神

> 注意：小耗、大耗、病符 在博士和岁前中重复出现，共用同一个 `StarName` 变体。
> 官符（岁前）/ 官府（博士）不同汉字，为不同枚举变体。

## 宫位固定位置 (PalacePos)

| 中文 | 拼音 | xstars 枚举 | 地支坐标 |
|------|------|------------|:--------:|
| 寅宫 | yin | `PalacePos::Yin` | 0 |
| 卯宫 | mao | `PalacePos::Mao` | 1 |
| 辰宫 | chen | `PalacePos::Chen` | 2 |
| 巳宫 | si | `PalacePos::Si` | 3 |
| 午宫 | wu | `PalacePos::Wu` | 4 |
| 未宫 | wei | `PalacePos::Wei` | 5 |
| 申宫 | shen | `PalacePos::Shen` | 6 |
| 酉宫 | you | `PalacePos::You` | 7 |
| 戌宫 | xu | `PalacePos::Xu` | 8 |
| 亥宫 | hai | `PalacePos::Hai` | 9 |
| 子宫 | zi | `PalacePos::Zi` | 10 |
| 丑宫 | chou | `PalacePos::Chou` | 11 |

## 十二宫 (Palace)

| 中文 | 拼音 | xstars 枚举 |
|------|------|------------|
| 命宫 | ming | `PalaceName::Fate` |
| 兄弟宫 | xiongdi | `PalaceName::Siblings` |
| 夫妻宫 | fuqi | `PalaceName::Spouse` |
| 子女宫 | zinv | `PalaceName::Children` |
| 财帛宫 | caibo | `PalaceName::Wealth` |
| 疾厄宫 | ji'e | `PalaceName::Health` |
| 迁移宫 | qianyi | `PalaceName::Travel` |
| 朋友宫 | pengyou | `PalaceName::Friends` |
| 官禄宫 | guanlu | `PalaceName::Career` |
| 田宅宫 | tianzai | `PalaceName::Property` |
| 福德宫 | fude | `PalaceName::Fortune` |
| 父母宫 | fumu | `PalaceName::Parents` |

## 主星 14 颗 (Major Stars)

| 中文 | 拼音 | xstars 枚举 | 五行 |
|------|------|------|------|
| 紫微 | Ziwei | `StarName::Ziwei` | 土 |
| 天机 | Tianji | `StarName::Tianji` | 木 |
| 太阳 | Taiyang | `StarName::Taiyang` | 火 |
| 武曲 | Wuqu | `StarName::Wuqu` | 金 |
| 天同 | Tiantong | `StarName::Tiantong` | 水 |
| 廉贞 | Lianzhen | `StarName::Lianzhen` | 火 |
| 天府 | Tianfu | `StarName::Tianfu` | 土 |
| 太阴 | Taiyin | `StarName::Taiyin` | 水 |
| 贪狼 | Tanlang | `StarName::Tanlang` | 水 |
| 巨门 | Jumen | `StarName::Jumen` | 水 |
| 天相 | Tianxiang | `StarName::Tianxiang` | 水 |
| 天梁 | Tianliang | `StarName::Tianliang` | 土 |
| 七杀 | Qisha | `StarName::Qisha` | 金 |
| 破军 | Pojun | `StarName::Pojun` | 水 |

## 辅星 14 颗 (Minor Stars)

| 中文 | 拼音 | xstars 枚举 | 五行 |
|------|------|------|------|
| 左辅 | Zuofu | `StarName::Zuofu` | 土 |
| 右弼 | Youbi | `StarName::Youbi` | 水 |
| 文昌 | Wenchang | `StarName::Wenchang` | 金 |
| 文曲 | Wenqu | `StarName::Wenqu` | 水 |
| 禄存 | Lucun | `StarName::Lucun` | 土 |
| 天马 | Tianma | `StarName::Tianma` | 火 |
| 天魁 | Tiankui | `StarName::Tiankui` | 火 |
| 天钺 | Tianyue | `StarName::Tianyue` | 火 |
| 擎羊 | Qingyang | `StarName::Qingyang` | 金 |
| 陀罗 | Tuoluo | `StarName::Tuoluo` | 金 |
| 火星 | Huoxing | `StarName::Huoxing` | 火 |
| 铃星 | Lingxing | `StarName::Lingxing` | 火 |
| 地空 | Dikong | `StarName::Dikong` | 火 |
| 地劫 | Dijie | `StarName::Dijie` | 火 |

## 四化 (Hua)

| 中文 | 拼音 | xstars 枚举 | 说明 |
|------|------|------|------|
| 禄 | Lu | `Hua::Lu` | 化禄 |
| 权 | Quan | `Hua::Quan` | 化权 |
| 科 | Ke | `Hua::Ke` | 化科 |
| 忌 | Ji | `Hua::Ji` | 化忌 |

## 亮度 (Brightness)

| 中文 | 拼音 | xstars 枚举 | 等级 |
|------|------|------|:----:|
| 庙 | Miao | `Brightness::Miao` | 1（最亮） |
| 旺 | Wang | `Brightness::Wang` | 2 |
| 得 | De | `Brightness::De` | 3 |
| 利 | Li | `Brightness::Li` | 4 |
| 平 | Ping | `Brightness::Ping` | 5 |
| 不 | Bu | `Brightness::Bu` | 6 |
| 陷 | Xian | `Brightness::Xian` | 7（最暗） |

## 流派 (School)

| 中文 | xstars 枚举 | 说明 |
|------|------------|------|
| 三合派 | `School::Sanhe` | 南派默认正统（《紫微斗数全书》） |
| 中州派 | `School::Zhongzhou` | 三合分支（王亭之），四化/命主/杂曜有变体 |
| 飞星派 | `School::Feixing` | 北派四化派，重四化飞宫 |

## 运限范围 (Layer)

| 中文 | xstars 枚举 | 说明 |
|------|------------|------|
| 基础盘 | `Palace.stars`（无运限层） | 一生不变的静态盘 |
| 大限 | `Layer::Major` | 十年大运 |
| 小限 | `Layer::Minor` | 一年小运 |
| 流年 | `Layer::Yearly` | 当年运限 |
| 流月 | `Layer::Monthly` | 当月运限 |
| 流日 | `Layer::Daily` | 当日运限 |
| 流时 | `Layer::Hourly` | 当时辰运限 |

## 语言 (Language)

| 中文 | xstars 枚举 | BCP47 |
|------|------------|-------|
| 简体中文 | `Language::ZhCN` | `zh-CN` |
| 繁体中文 | `Language::ZhTW` | `zh-TW` |
| 英语 | `Language::EnUS` | `en-US` |
| 日语 | `Language::JaJP` | `ja-JP` |
| 韩语 | `Language::KoKR` | `ko-KR` |
| 越南语 | `Language::ViVN` | `vi-VN` |

## 建月（寅视图月建）

| 建月 | 对应地支 | xstars 表示 | 农历月份 | 寅视图索引 |
|------|---------|------------|---------|:--------:|
| 寅月 | 寅 | — | 正月 | 0 |
| 卯月 | 卯 | — | 二月 | 1 |
| 辰月 | 辰 | — | 三月 | 2 |
| 巳月 | 巳 | — | 四月 | 3 |
| 午月 | 午 | — | 五月 | 4 |
| 未月 | 未 | — | 六月 | 5 |
| 申月 | 申 | — | 七月 | 6 |
| 酉月 | 酉 | — | 八月 | 7 |
| 戌月 | 戌 | — | 九月 | 8 |
| 亥月 | 亥 | — | 十月 | 9 |
| 子月 | 子 | — | 十一月 | 10 |
| 丑月 | 丑 | — | 十二月 | 11 |

> 注：节气映射未使用独立枚举，`Dizhi::yin_coord()` 方法将节气地支转为对应月建（寅→正月，卯→二月...）。

## 三方四正 (CastPalaces)

| 中文 | 英文 | xstars 字段 |
|------|------|------------|
| 本宫 | Origin palace | `CastPalaces::origin` |
| 左三合（财帛） | Left tripartite | `CastPalaces::left` |
| 对宫（迁移） | Opposite palace | `CastPalaces::opposite` |
| 右三合（官禄） | Right tripartite | `CastPalaces::right` |

```rust
// 查询方法
cast.has_any(&[StarName::Ziwei])                  // 任一宫含任一星
cast.has_all(&[StarName::Ziwei, StarName::Tianfu])  // 全部星均匀分布
cast.contains_hua(Hua::Lu)                                    // 含化禄
cast.not_have(&[StarName::Dikong])                  // 无一宫含
!cast.contains_hua(Hua::Ji)                               // 无化忌
```

## 运限结构体 (Yunxian)

| 结构体 | xstars 类型 | 说明 |
|--------|------------|------|
| 运限 | `Yunxian` | 完整运限数据（大限/小限 + 流年/月/日/时） |
| 运限层 | `YunxianLayer` | 单层运限（流年/流月/流日/流时共用） |
| 小限年龄列 | `Vec<usize>` | 该宫位小限包含的年龄列表 |

**YunxianLayer 字段**：

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `String` | 层名（如"丙午"） |
| `pillar` | `(Tiangan, Dizhi)` | 运限柱（天干地支） |
| `pos` | `PalacePos` | 运限起宫位置 |
| `palace_names` | `Vec<String>` | 12 宫在本层的宫名 |
| `hua` | `[(StarName, Hua); 4]` | 运限四化（固定 4 元数组） |
| `stars` | `Vec<Vec<Star>>` | 运限星（12 宫，按宫位坐标索引） |

## 格局 (Pattern)

| 中文 | xstars 类型 | 说明 |
|------|------------|------|
| 格局 | `Pattern` | 星曜组合形成的特殊命理格局 |
| 格局等级 | `PatternLevel` | Excellent / Good / Neutral / Caution |
| 格局条件 | `PatternCondition` | Required / Bonus / Breaking |
| 格局检测 | `detect_patterns(a)` | 返回匹配的所有格局列表 |

当前收录 **54 个格局**，源码在 `astro/pattern/`。

## 飞星 API

| 方法 | 说明 |
|------|------|
| `a.fly_hua(pos)` | 返回本宫四化飞入的目标宫位 |
| `a.fly_from(star, hua)` | 返回将指定四化飞向指定星曜的宫位 |
| `palace.self_hua()` | 返回本宫自化星曜和四化 |
| `palace.hua(Hua::Lu)` | 查询本宫天干对应的四化星 |

## 配置项 (AppConfig)

| 配置项 | 选项 | 缺省 | 说明 |
|--------|------|:---:|------|
| `year_divide` | `Lichun` / `Chunjie` | `Chunjie` | 岁分起始 |
| `day_divide` | `Shift` / `Keep` | `Shift` | 日时分界 |
| `leap_month` | `Keep` / `Shift` / `Split` | `Split` | 闰月规则 |
| `major_period_divide` | `Lichun` / `Birthday` / `Chunjie` | `Chunjie` | 大限起算 |
| `minor_period_divide` | `Lichun` / `Birthday` / `Chunjie` | `Chunjie` | 小限起算 |
| `hua_table` | `HuaTable`（可自定义） | `HuaTable::DEFAULT` | 四化表 |
| `mingzhu_rule` | `ByYearDizhi` / `ByFateDizhi` | `ByFateDizhi` | 命主规则 |
| `tianshi_rule` | `Fixed` / `YinYangSwap` | `Fixed` | 天使天伤规则 |
| `month_rule` | `Lunar` / `Jieqi` | `Lunar` | 月支来源 |
| `misc_star_set` | `JieluKongwang` / `LongdeJiekong` | `JieluKongwang` | 杂曜配置 |
| `suiqian_variant` | `Dahao` / `Suipo` | `Dahao` | 岁前神煞变体 |
| `star_scope` | `Full` / `Simplified` | `Full` | 星曜数量 |
| `brightness` | `BrightnessTable`（可自定义） | `BrightnessTable::DEFAULT` | 星曜亮度表 |
| `age_divide` | `Nominal` / `Birthday` | `Nominal` | 虚岁计算方式 |
