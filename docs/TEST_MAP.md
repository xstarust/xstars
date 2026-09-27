# 测试地图

> xstars 项目测试全览：测试目标、条件、期望值与入口索引。
>
> **统计**: 234 个运行时测试 PASS、0 ignored；另有 26 个 doctest PASS
>
> 数据来源：`cargo test -p xstars --all-targets`（82 个单元测试 + 152 个集成测试）和 `cargo test -p xstars --doc`（26 个 doctest）。
> **Lint**: `cargo clippy -- -D warnings` ✅

---

## 目录

1. [集成测试](#1-集成测试)
   - [1.1 星曜算法 (`star_test.rs`)](#11-星曜算法-star_testrs)
   - [1.2 辅星定位 (`location_test.rs`)](#12-辅星定位-location_testrs)
   - [1.3 宫位系统 (`palace_test.rs`)](#13-宫位系统-palace_testrs)
   - [1.4 分析功能 (`analyzer_test.rs`)](#14-分析功能-analyzer_testrs)
   - [1.5 星盘基础 (`astrolabe_test.rs`)](#15-星盘基础-astrolabe_testrs)
   - [1.6 大限小限 (`horoscope_test.rs`)](#16-大限小限-horoscope_testrs)
   - [1.7 历法计算 (`calendar_test.rs`)](#17-历法计算-calendar_testrs)
   - [1.8 流派配置 (`config_test.rs`)](#18-流派配置-config_testrs)
   - [1.9 工具函数 (`utils_test.rs`)](#19-工具函数-utils_testrs)
   - [1.10 国际化 (`i18n_test.rs`)](#110-国际化-i18n_testrs)
   - [1.11 格局识别 (`pattern.rs`)](#111-格局识别-patternrs)
   - [1.12 运限层 (`yunxian_test.rs`)](#112-运限层-yunxian_testrs)
2. [单元测试](#2-单元测试)
   - [2.1 天干 (`system/tiangan.rs`)](#21-天干-systemtianganrs)
   - [2.2 地支 (`system/dizhi.rs`)](#22-地支-systemdizhirs)
   - [2.3 时辰 (`system/shichen.rs`)](#23-时辰-systemshichenrs)
   - [2.4 宫位坐标 (`astro/palace_pos.rs`)](#24-宫位坐标-astropalace_posrs)
   - [2.5 宫位算法 (`astro/palace.rs`)](#25-宫位算法-astropalacers)
   - [2.6 历法 (`calendar/mod.rs`)](#26-历法-calendarmodrs)
   - [2.7 工具索引 (`util/index.rs`)](#27-工具索引-utilindexrs)
   - [2.8 杂曜定位 (`star/misc.rs`)](#28-杂曜定位-starmiscrs)
3. [测试夹具](#3-测试夹具)
4. [覆盖率分析](#4-覆盖率分析)

---

## 1. 集成测试

**入口**: `tests/`

### 1.1 星曜算法 (`star_test.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 紫微天府索引 | `2023-08-01`, 女, timeIndex=0/2/4 | 子丑同宫时紫微在天府对面；时支变化后索引正确 | `test_ziwei_tianfu_indices` |
| 2 | 主星排布 | `2023-03-06`, timeIndex=4, 女 | 十二宫主星分布匹配预期（含紫微天府同宫申位） | `test_major_star_placement_20230306_t4` |
| 3 | 主星四化 | `2023-03-06`, timeIndex=4, 女 | 破军化禄、太阴化科、贪狼化忌、巨门化权 | (同上，内联断言) |
| 4 | 辅星数量 | `2023-03-06`, timeIndex=4, 女 | 共 14 颗辅星 | `test_minor_star_count` |
| 5 | 杂曜数量 | `2023-03-06`, timeIndex=4, 女 | ≥ 6 颗 | `test_misc_star_count` |
| 6 | 红鸾天喜数量 | `2023-03-06`, timeIndex=4, 女 | 红鸾、天喜共 2 颗 | `test_dust_star_count` |
| 7 | 十四主星完整 | `2023-03-06`, timeIndex=4, 女 | 紫微、天机、太阳、武曲、天同、廉贞、天府、太阴、贪狼、巨门、天相、天梁、七杀、破军全部出现 | `test_all_14_major_stars_present` |
| 8 | r1 主星总数 | fixture `2023-8-15` t0 女 | 14 颗主星（含空宫） | `test_major_stars_for_r1` |
| 9 | 五行局长生起始 | 水二局→6, 木三局→9, 金四局→3, 土五局→6, 火六局→0 | 长生十二神的起始宫位坐标 | `test_changsheng_start_indices` |
| 10 | 将前十二神起始 | 寅午戌→午4, 申子辰→子10, 巳酉丑→酉7, 亥卯未→卯1 | 三合局对应的将星宫位 | `test_get_jiangqian_start_pos_indices` |
| 11 | 主星亮度 | `2023-03-06`, timeIndex=4, 女 | 14 主星各自亮度：七杀庙、天同平、武曲庙、太阳旺、破军庙、天机陷、紫微旺、天府得、太阴不、贪狼庙、巨门旺、廉贞平、天相庙、天梁旺 | `test_major_star_brightness_20230306_t4` |
| 12 | 长生十二神顺序 | 常量表 | 长生→沐浴→冠带→临官→帝旺→衰→病→死→墓→绝→胎→养 | `test_changsheng_12_order` |
| 13 | 博士十二神顺序 | 常量表 | 长度 12，含博士/青龙/力士/官府 | `test_boshi_12_order` |
| 14 | 博士十二神排盘 | `2023-8-15` 女，癸卯年阴女顺行 | 青龙→小耗→将军→奏书→飞廉→喜神→病符→大耗→伏兵→官府→博士→力士 | `test_boshi_12_placement` |
| 15 | 全盘星曜总数 | `2023-03-06`, timeIndex=4, 女 | ≥ 30 | `test_total_stars_count` |
| 16 | 辅星合并总数 | `2023-03-06`, timeIndex=4, 女 | ≥ 66（主星+辅星+杂曜+合计） | `test_minor_star_merged_count` |
| 17 | 运限星-大限 | 庚干辰支 Decadal | 12 宫运限星：运马、运曲、运喜、运钺+运陀、运禄、运羊、运昌+运鸾、运魁 | `test_horoscope_star_decadal` |
| 18 | 长生十二神排盘 | `2023-8-15` t0 女（土五局起申6, 阴女逆行） | 标准正向顺序 | `test_changsheng_12_placement` |
| 19 | 长生十二神(女) | `1999-5-3` t8 女（金四局起巳3, 阴女逆行） | 绝→胎→养→长生→沐浴→冠带→临官→帝旺→衰→病→死→墓 | `test_changsheng_12_placement_female` |
| 20 | 长生十二神(from 参数) | 丙子覆盖，阴年男逆行 | 病→衰→帝旺→临官→冠带→沐浴→长生→养→胎→绝→墓→死 | `test_changsheng_12_placement_from_param` |
| 21 | 岁前/将前十二神 | 2025 乙巳年 | 岁前：天德/吊客/病符/岁建/晦气/丧门/贯索/官符/小耗/大耗/龙德/白虎；将前：劫煞/灾煞/天煞/指背/咸池/月煞/亡神/将星/攀鞍/岁驿/息神/华盖 | `test_yearly_12_2025` |
| 22 | 岁前/将前十二神 | 2023 癸卯年 | 岁前：病符始 将前：亡神始 | `test_yearly_12_2023` |

- 杂曜定位集成验证 | `2001-08-16` t2 男 | 含截路、空亡、年解、天使、天伤 | `test_adjective_star_2001_08_16` |

---

### 1.2 辅星定位 (`location_test.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 禄存/擎羊/陀罗/天马 | r1: `2023-8-15` t0 女 | 禄存亥10, 擎羊子11, 陀罗戌9, 天马巳3 | `test_lu_yang_tuo_ma_kui_mao` |
| 2 | 禄存/擎羊/陀罗/天马 | `2010-3-1` t0 女（庚寅年） | 禄存申6, 擎羊酉7, 陀罗未5, 天马申6 | `test_lu_yang_tuo_ma_geng_yin` |
| 3 | 天魁/天钺 | `1984-2-20` t0 女（甲子年） | 天魁亥11, 天钺辰5 | `test_kui_yue_jia_year` |
| 4 | 天魁/天钺 | `1992-2-20` t0 女（壬申年） | 天魁卯1, 天钺巳3 | `test_kui_yue_ren_year` |
| 5 | 文昌/文曲 | `2023-8-15` t0 女 | 文昌酉8, 文曲巳2 | `test_chang_qu_time_0` |
| 6 | 文昌/文曲 | `2023-8-15` t2 男 | 文昌未6, 文曲巳4 | `test_chang_qu_time_2` |
| 7 | 地空/地劫 | `2023-8-15` t0 女 | 地空亥9, 地劫亥9 | `test_kong_jie_time_0` |
| 8 | 地空/地劫 | `2023-8-15` t4 男 | 地空申5, 地劫辰1 | `test_kong_jie_time_4` |
| 9 | 红鸾/天喜 | r1: `2023-8-15` t0 女 | 红鸾亥10, 天喜巳4 | `test_hongluan_tianxi` |
| 10 | 台辅/封诰 | `2023-8-15` t0 女 | 台辅巳4, 封诰寅0 | `test_taifu_fenggao` |
| 11 | 三台/八座 | `2020-8-5` t0 男，农历十六日 | 三台子0, 八座寅2 | `test_daily_star_index` |

---

### 1.3 宫位系统 (`palace_test.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 五行局 12 组 | 庚申→木三局, 己未→火六局, 戊午→火六局, 丁巳→土五局, 丙辰→土五局, 乙卯→水二局, 甲寅→水二局, 乙丑→金四局, 甲子→金四局, 癸亥→水二局, 壬戌→水二局, 辛酉→木三局 | 12 对天干地支映射正确 | `test_wuxing_group_*` (12 tests) |
| 2 | 命宫身宫 | `2023-1-22` t5 | 命宫酉7, 身宫巳5, 命干己, 命支酉 | `test_soul_and_body_2023_01_22_t5` |
| 3 | 命宫身宫 | `2023-1-22` t6 | 命宫申6, 身宫申6, 命干戊, 命支申 | `test_soul_and_body_2023_01_22_t6` |
| 4 | 命宫身宫(晚子时) | `2023-2-19` t12 | 命宫寅0, 身宫寅0, 命干甲, 命支寅 | `test_soul_and_body_2023_02_19_t12` |
| 5 | 命宫身宫范围 | 3 个用例 | 结果索引均 < 12 | `test_soul_and_body_case_[123]` |
| 6 | 宫名序列 | 命宫索引=1 | 返回 12 宫名 | `test_palace_names_returns_12_elements` |
| 7 | 宫名序列边界 | 命宫索引 1/13/-11 | 循环结果一致 | `test_palace_names_boundary` |
| 8 | 大限小限-女 | `2023-11-15` t3 女（阴水→阳女逆行） | 小限组每组 10 个年龄，丑宫含 1 岁 | `test_horoscope_ages_female_explicit` |
| 9 | 大限小限-男 | `2023-11-15` t3 男（阳水→阳男顺行） | 小限组每组 10 个年龄，丑宫含 1 岁 | `test_horoscope_ages_male_explicit` |

---

### 1.4 分析功能 (`analyzer_test.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 宫位索引查询 | r1 | 通过索引获取宫位，名称不为空 | `test_palace_by_index` |
| 2 | 宫位名称查询 | r1 | 通过名称获取宫位与索引一致 | `test_palace_by_name` |
| 3 | 宫位索引越界 | r1 | index=12 panic | `test_palace_12_panics` |
| 4 | 宫位索引越界 | r1 | index=-1 panic | `test_palace_minus1_panics` |
| 5 | 宫位星曜 `has_any` | r1 | 财帛没有太阳、有天相；子女有天机或天梁 | `test_palace_has_stars` |
| 6 | 宫位无星曜 | r1 | 财帛检查太阳为 false | `test_palace_not_have_stars` |
| 7 | 宫位包含之一 | r1 | 财帛检查太阳或天相为 true | `test_palace_has_one_of_stars` |
| 8 | 三方四正 `have_any` | r1 | 命宫三方包含武曲/贪狼/擎羊/天相/天魁/地空/地劫等 | `test_cast_have_any` |
| 9 | 三方四正 `have_one_of` | r2 | 命宫三方有太阳或文曲 | `test_cast_have_one_of` |
| 10 | 三方四正 `not_have` | r2 | 命宫三方无地空地劫 | `test_cast_not_have` |
| 11 | `is_cast` | r1 | 命宫四方分布有目标星列 | `test_is_cast` |
| 12 | `is_cast_one_of` | r2 | 命宫四方有太阳或文曲 | `test_is_cast_one_of` |
| 13 | 空宫判断 | r1 | 存在含主星的宫位 | `test_is_empty_palace_has_stars` |
| 14 | 三方四正 by index | r1 | 寅宫三方四正存在 | `test_cast_palaces_by_index` |
| 15 | 三方四正/命宫 | r1 | target 恒等于命宫 | `test_cast_palaces_ming_gong` |

---

### 1.5 星盘基础 (`astrolabe_test.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | Astrolabe 创建 | `2023-11-15` t3 女 | 12 宫位、农历年非零 | `test_astrolabe_creation` |
| 2 | 基础属性 | r4 `2000-8-16` t2 女 | 女性、时支=寅2、命身宫 < 12 | `test_astrolabe_properties` |
| 3 | 命宫索引 | r5 `2023-11-15` t3 女 | 索引 < 12 | `test_astrolabe_soul_index` |
| 4 | 三方四正-寅 | r1 | target=寅, opposite=申, wealth=戌, career=午 | `test_cast_palaces_index_0` |
| 5 | 三方四正-命宫 | r1 | target=命宫位置 | `test_cast_palaces_index_5` |
| 6 | 三方四正-全部 | r4 | 12 宫均有效 | `test_cast_palaces_all_indices` |
| 7 | 空宫判断 | r1 | 存在含主星的宫位 | `test_is_empty_palace_has_stars` |
| 8 | 宫位星曜非空 | r4 | 所有星曜名称非空串 | `test_star_names_not_empty` |
| 9 | 星曜查找-找到 | r1 | 紫微找到且索引 < 12 | `test_star_lookup_found` |
| 10 | 星曜查找-未找到 | r1 | 不存在星曜返回 None | `test_star_lookup_not_found` |
| 11 | 星曜查找-多宫 | r1 | 红鸾找到且名称匹配 | `test_star_lookup_multiple_palaces` |
| 12 | 农历排盘 Roundtrip | `2000-7-17` 农历与 `2000-8-16` 阳历 | 命宫/身宫/五行局一致，命宫主星一致 | `test_by_lunar_roundtrip` |
| 13 | 农历排盘属性 | `2000-7-17` 农历 t2 女 | 12 宫，命宫非空 | `test_by_lunar_properties` |

---

### 1.6 大限小限 (`horoscope_test.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 大限数量(女) | `2023-11-15` t3 女 | 12 组大限 | `test_major_cycles_female_basic` |
| 2 | 大限年龄连续 | `2023-11-15` t3 女 | 每组起始≤结束，各组连续 | `test_major_cycles_age_range_continuous` |
| 3 | 大限数量(男) | `2023-11-15` t3 男 | 12 组大限 | `test_major_cycles_male_count` |
| 4 | 男女大限长度一致 | 同上日期不同性别 | 大限组数相同 | `test_major_cycles_same_input_diff_gender` |
| 5 | 大限-不同日期(女) | `2000-8-16` t2 女 | 12 组大限 | `test_major_cycles_f2_basic` |
| 6 | 小限数量(女) | `2023-11-15` t3 女 | 12 组，每组 10 个年龄 | `test_ages_female_basic` |
| 7 | 小限数量(男) | `2023-11-15` t3 男 | 12 组，每组 10 个年龄 | `test_ages_male_basic` |
| 8 | 小限年龄递增 | `2023-11-15` t3 女 | 每组年龄严格递增 | `test_ages_increasing` |
| 9 | 小限-不同日期(女) | `2000-8-16` t2 女 | 12 组，每组 10 个年龄 | `test_ages_f2_basic` |

---

### 1.7 历法计算 (`calendar_test.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 阳历转农历 | `2000-8-16` | 农历 2000-07-17（非闰月） | `test_solar_to_lunar` |
| 2 | 农历转阳历 | `2000-7-17` 农历 | `2000-8-16` | `test_lunar_to_solar` |
| 3 | 阳历转农历多日期 | `2023-1-22`/`2023-8-15`/`2023-12-31` t0 | 均为 2023 农历年 | `test_solar_to_lunar_multiple` |
| 4 | 闰月处理 | `2023-3-22` | 农历年非零 | `test_leap_month` |

---

### 1.8 流派配置 (`config_test.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 三合派四化表-全 10 干 | 甲→廉贞禄/破军权/武曲科/太阳忌；戊→贪狼禄/太阴权/右弼科/天机忌；庚→太阳禄/武曲权/太阴科/天同忌；壬→天梁禄/紫微权/左辅科/武曲忌；癸→破军禄/巨门权/太阴科/贪狼忌 | 10 天干映射全部正确 | `test_sanhe_hua_table_all_10_stems` |
| 2 | 中州派四化差异 | 戊科右弼→太阳；庚科太阴→天府；壬科左辅→天府 | 7 干不变，3 干化科不同 | `test_zhongzhou_hua_table_differences` |
| 3 | 三合派预设星曜 | `2000-8-16` t2 女 school=sanhe | 含截路、空亡、天使、天伤 | `test_sanhe_preset_has_expected_stars` |
| 4 | 中州派预设星曜 | `2000-8-16` t2 女 school=zhongzhou | 含龙德、截空；无截路、空亡 | `test_zhongzhou_preset_has_longde_jiekong` |
| 5 | 飞星派 Simplified | `2000-8-16` t2 女 school=feixing | 无华盖/孤辰/红鸾/天喜；有紫微 | `test_feixing_simplified_no_misc_stars` |
| 6 | Builder 维度覆盖 | 中州四化+阴阳互换+留闰+立春分年 | 覆盖生效，未覆盖保持默认 | `test_builder_dimension_overrides` |
| 7 | 三合≡默认 | 默认配置 vs 三合预设 | 年支分界、天使规则、杂曜集、星曜范围一致 | `test_sanhe_equals_default` |
| 8 | 飞星≈三合 scope 不同 | 三合 vs 飞星 | 飞星 scope=Simplified | `test_feixing_from_sanhe_only_scope_differs` |
| 9 | 中州≠三合 | 三合 vs 中州 | 年支分界/天使规则/杂曜集/岁前变体/闰月均不同 | `test_zhongzhou_differs_from_sanhe` |
| 10 | 地盘—身宫为命宫 | 天盘身宫→地盘命宫 | 地盘原身宫位名为命宫、原命宫位非命宫 | `test_astro_layer_di_uses_body_as_fate` |
| 11 | 人盘—福德宫为命宫 | 天盘命宫+2→人盘命宫 | 人盘原福德宫位名为命宫 | `test_astro_layer_ren_uses_fortune_as_fate` |
| 12 | 岁前-中州(岁破) | 巳年 SuiqianVariant::Suipo | 含岁破不含大耗 | `test_suiqian_zhongzhou_uses_suipo` |
| 13 | 岁前-三合(大耗) | 巳年 SuiqianVariant::Dahao | 含大耗不含岁破 | `test_suiqian_sanhe_uses_dahao` |
| 14 | 自定义四化表 | 全指向紫微的自定义表 | 甲干四化均紫微 | `test_custom_hua_table` |

---

### 1.9 工具函数 (`utils_test.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | `wrap_idx` 正值 | 1/11/15 | 返回原值 | `test_wrap_idx_positive` |
| 2 | `wrap_idx` 负值 | -2/-3/-13/-2/-3/-15 | 正向映射 | `test_wrap_idx_negative` |
| 3 | `wrap_idx` 溢出 | 12/13/23/23 | mod 归位 | `test_wrap_idx_overflow` |
| 4 | 地支→宫位坐标 | 子0→10, 丑1→11, 寅2→0, 卯3→1, 辰4→2, 巳5→3, 午6→4, 未7→5, 申8→6, 酉9→7, 戌10→8, 亥11→9 | 公式 `(branch + 10) % 12` | `test_branch_to_palace_coord` |
| 5 | 天干枚举 | 甲→癸, 含拼音反查 | 10 干中英文/索引一致，`from_str` 双向解析 | `test_tiangan_enum` |
| 6 | 地支枚举 | 子→亥, 含拼音反查 | 12 支中英文/索引一致，`from_str` 双向解析 | `test_dizhi_enum` |
| 7 | `time_to_index` | 0→0, 1→1, 2→1, 3→2 ... 23→12 | 24 小时→13 时辰索引 | `test_time_to_index` |
| 8 | `age_index` 小限起始 | 寅午戌→辰2, 申子辰→戌8, 巳酉丑→未5, 亥卯未→丑11 | 三合局→小限起始宫位 | `test_age_index` |

---

### 1.10 国际化 (`i18n_test.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 韩语星盘输出 | 韩语语言环境 | 星盘关键字段可正常生成 | `test_korean_astrolabe` |
| 2 | 越南语星盘输出 | 越南语语言环境 | 星盘关键字段可正常生成 | `test_vietnamese_astrolabe` |

### 1.11 格局识别 (`pattern.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 格局检测 | r4 `2000-8-16` t2 女 | 至少触发 1 个格局 | `test_detect_patterns_r4` |
| 2 | 不同输入不同格局 | r4 vs `1990-5-15` t4 男 | 格局列表不同 | `test_detect_patterns_different_inputs` |
| 3 | 等级顺序 | r4 | 第一个格局至少中格 | `test_pattern_level_order` |
| 4 | 古籍出处 | r4 | 所有格局均有 `source` | `test_pattern_source_not_empty` |
| 5 | 稳定性 | r4 重复检测 | 格局列表一致 | `test_patterns_stable` |

古籍依据：《紫微斗数全书》《紫微斗数骨髓赋》。共 54 个格局，每个有 required/bonus/breaking 三层条件。

---

### 1.12 运限层 (`yunxian_test.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 五层运限索引/柱 | `2000-8-16` t2 女 → `horoscope('2023-8-19 3:12')` | decadal=2庚辰, yearly=1癸卯, monthly=3庚申, daily=6己酉, hourly=8丙寅, age=24 | `test_yunxian_basic_index_pillar` |
| 2 | 流年十二神 | `2000-8-16` t2 女 → 2023 癸卯年 | 岁前：病符/岁建/晦气/丧门/贯索/官符/小耗/大耗/龙德/白虎/天德/吊客；将前：亡神/将星/攀鞍/岁驿/息神/华盖/劫煞/灾煞/天煞/指背/咸池/月煞 | `test_yunxian_suiqian_jiangqian` |
| 3 | 不同日期运限 | `1991-3-7` t6 女 → `horoscope('2025-3-26')` | decadal=8戊戌, yearly=3乙巳, monthly=10己卯, daily=0甲午 | `test_yunxian_horoscope_1991` |
| 4 | 流年运限星排布 | 癸9卯3 scope=yearly | [1]流魁流昌 [3]流钺流马 [4]流喜 [5]年解 [9]流曲流陀 [10]流禄流鸾 [11]流羊 | `test_yunxian_yearly_star_placement` |
| 5 | 命宫在流年层 | `2000-8-16` t2 女 → 2023 癸卯 | 命宫(午)在流年层对应福德宫 | `test_palace_at_yearly_scope` |
| 6 | 六层结构完整性 | 大限/小限/流年/流月/流日/流时 | 每层 12 宫名 + 4 组四化 | `test_yunxian_layer_structure` |

---

## 2. 单元测试

**入口**: `src/` 内 `#[cfg(test)]` 模块，共 **82** 测试。

### 2.1 天干 (`system/tiangan.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 输出中文 | `Tiangan::Jia.to_str()` | "甲" | `test_tiangan_chinese` |
| 2 | 阴阳属性 | `Tiangan::Jia.yinyang()` | Yang | `test_tiangan_yinyang` |
| 3 | 阳干判断 | `Tiangan::Jia.is_yang()` | true | `test_tiangan_is_yang` |
| 4 | 整型构造 | `Tiangan::from(0u8)` | Jia | `test_tiangan_from_integers` |
| 5 | 索引 | `Tiangan::Jia.index()` | 0 | `test_tiangan_index` |

### 2.2 地支 (`system/dizhi.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 输出中文 | `Dizhi::Zi.to_str()` | "子" | `test_dizhi_chinese` |
| 2 | 顺移 | `Dizhi::Zi.forward(1)` | Chou | `test_forward` |
| 3 | 逆移 | `Dizhi::Zi.backward(1)` | Hai | `test_backward` |
| 4 | 对宫 | `Dizhi::Zi.opposite()` | Wu | `test_opposite` |
| 5 | 三合局 | `Dizhi::Shen.sanhe()` | 申子辰三合 | `test_sanhe` |
| 6 | 整型构造 | `Dizhi::from(0u8)` | Zi | `test_from_integers` |
| 7 | Add(usize) | `Dizhi::Yin + 1` | Mao | `test_add_usize` |
| 8 | Sub(usize) | `Dizhi::Zi - 1` | Hai | `test_sub_usize` |
| 9 | Add(isize) | `Dizhi::Yin + 1` | Mao | `test_add_isize` |

### 2.3 时辰 (`system/shichen.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 时间→时辰 | `"0:00"` | Zi (子) | `test_from_hour` |
| 2 | 时间(含分) | `"00:30"` | Zi | `test_from_hour_minute` |
| 3 | 索引构造 | `Shichen::from_index(0)` | Zi | `test_from_index` |
| 4 | 索引 | `Shichen::Zi.index()` | 0 | `test_index` |
| 5 | 地支转换 | `Shichen::Zi.dizhi()` | Dizhi::Zi | `test_dizhi` |

### 2.4 宫位坐标 (`astro/palace_pos.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 索引 | `PalacePos::Yin.index()` | 0 | `test_index` |
| 2 | 地支转换 | `PalacePos::Yin.dizhi()` | Dizhi::Yin | `test_dizhi` |
| 3 | 顺移 | `PalacePos::Yin.forward(1)` | Mao | `test_forward` |
| 4 | 逆移 | `PalacePos::Yin.backward(1)` | Chou | `test_backward` |
| 5 | 对宫 | `PalacePos::Zi.opposite()` | Wu | `test_opposite` |
| 6 | Add(usize) | `PalacePos::Yin + 1` | Mao | `test_add` |
| 7 | Sub(usize) | `PalacePos::Yin - 1` | Chou | `test_sub` |

### 2.5 宫位算法 (`astro/palace.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 命宫身宫 | `2023-1-22` t5 | 命宫酉7, 身宫巳5 | `test_soul_and_body` |
| 2 | 宫名序列-12 | 任意入参 | 返回 12 个宫名 | `test_palace_names_returns_12_elements` |
| 3 | 宫名序列-边界 | 1/13/-11 入参 | 循环一致 | `test_palace_names_boundary` |

### 2.6 历法 (`calendar/mod.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | Builder 默认 | 无参构建 | 默认值有效 | `test_builder_default` |
| 2 | Builder 时辰 | 显式指定 Shichen | 时柱正确 | `test_builder_with_shichen` |
| 3 | 四柱 | 完整日期+时辰 | 年/月/日/时四柱均有效 | `test_four_pillars` |
| 4 | 农历日期字符串 | 阳历→农历 | 格式 "2000-07-17" | `test_lunar_date_string` |
| 5 | 农历→阳历 | `"2000-7-17"` | `"2000-8-16"` | `test_lunar_to_solar` |
| 6 | 阳历→农历→阳历 Roundtrip | 多日期 | 往返一致 | `test_to_solar_roundtrip` |
| 7 | 晚子时 | 23:00~23:59 | 次日时柱（癸亥日→甲子日） | `test_wanzi_hour` |

### 2.7 工具索引 (`util/index.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | `wrap_idx` 正值 | 0 | 0 | `test_wrap_idx_positive` |
| 2 | `wrap_idx` 负值 | -1 → 11 | 正向循环 | `test_wrap_idx_negative` |
| 3 | `wrap_idx` 溢出 | 25 → 1 | mod 归位 | `test_wrap_idx_overflow` |
| 4 | `time_to_index` 全天 | 0→0, 1→1, ... | 24h→时辰 | `test_time_to_index` |
| 5 | `age_index` | 三合局→小限起始 | 寅/午/戌→辰2 | `test_age_index` |
| 6 | 地支枚举 | 12 支全循环 | 中文拼音/索引/`from_str` 正确 | `test_dizhi_enum` |
| 7 | 天干枚举 | 10 干全循环 | 中文拼音/索引/`from_str` 正确 | `test_tiangan_enum` |
| 8 | 地支→宫位坐标 | 12 支全转换 | 公式 `(i + 10) % 12` | `test_branch_to_palace_coord` |

### 2.8 杂曜定位 (`star/misc.rs`)

| # | 测试目标 | 测试条件 | 测试期望 | 入口 |
|---|----------|----------|----------|------|
| 1 | 华盖咸池 | 申8 | 返回位置对 (华盖, 咸池) | `test_huagai_xianchi` |
| 2 | 孤辰寡宿 | 寅2 | 返回位置对 (孤辰, 寡宿) | `test_gu_gua` |
| 3 | 天才天寿 | — | 返回位置对 (天才, 天寿) | `test_tiancai_tianshou` |
| 4 | 天厨 | 甲0 | 宫位索引 3 | `test_tianchu` |
| 5 | 破碎 | 子0 | 宫位索引 3 | `test_posui` |
| 6 | 蜚廉 | 子0 | 宫位索引 6 | `test_feilian` |
| 7 | 龙池凤阁 | 子0 | 返回位置对 (龙池, 凤阁) | `test_longchi_fengge` |
| 8 | 天哭天虚 | 子0 | 返回位置对 (天哭, 天虚) | `test_tianku_tianxu` |
| 9 | 天官 | 甲0 | 宫位索引 5 | `test_tianguan` |
| 10 | 天福 | 甲0 | 宫位索引 7 | `test_tianfu_fortune` |
| 11 | 天德 | 子0 | 宫位索引 7 | `test_tiande` |
| 12 | 月德 | 子0 | 宫位索引 3 | `test_yuede` |
| 13 | 天空 | 子0 | 宫位索引 11 | `test_tiankong` |
| 14 | 截路空亡 | 甲0 | 返回位置对 (截路, 空亡) | `test_jielu_kongwang` |
| 15 | 旬空 | 子0, 甲0 | 返回旬空位置 | `test_xunkong` |
| 16 | 年解 | 子0 | 宫位索引 8 | `test_nianjie` |
| 17 | 劫煞 | 子0 | 宫位索引 3 | `test_jiesha` |

---

## 3. 测试夹具

**文件**: `tests/common/mod.rs`

| 名称 | 定义 | 用途 |
|------|------|------|
| `fixture!(date, time, gender)` | 宏 — `Astrolabe::builder(d, t, g).build().unwrap()` | 快速创建测试星盘 |
| `r1()` | `2023-8-15`, t0, 女 | 命宫在午，通用测试基准 |
| `r2()` | `2023-8-16`, t2, 女 | 分析测试基准 |
| `r3()` | `2013-8-21`, t4, 女 | 补充测试 |
| `r4()` | `2000-8-16`, t2, 女 | Astrolabe 属性测试 |
| `r5()` | `2023-11-15`, t3, 女 | 大限测试 |
| `r6()` | `2023-11-15`, t3, 男 | 大限测试(性别对比) |
| `star_palace_index(astro, cn)` | 查找星曜所在宫位索引 | 辅星定位测试 |
| `palace_stars_cn(astro, idx)` | 获取宫位星曜列表 | 辅助函数 |

---

## 4. 覆盖率分析

### 覆盖的功能域

| 功能域 | 覆盖度 | 说明 |
|--------|--------|------|
| 天干地支 | ✅ 完备 | 枚举、运算、显示、解析全覆盖 |
| 时辰系统 | ✅ 完备 | 时→辰转换、索引、地支映射 |
| 宫位坐标 (PalacePos) | ✅ 完备 | 索引、顺逆、对宫、算术 |
| 五行局 | ✅ 完备 | 12 组天干地支→五行局 |
| 命宫身宫 | ✅ 完备 | 三个关键日期 + 晚子时 |
| 紫微天府索引 | ✅ 完备 | 三个时辰用例 |
| 十四主星排布 | ✅ 完备 | 全星列表 + 特定日期排布 |
| 主星亮度 | ✅ 完备 | 14 主星亮度对照参考实现 |
| 四化 | ✅ 完备 | 合参用例 + 三合/中州表 |
| 辅星定位 (14 颗) | ✅ 完备 | 禄存/擎羊/陀罗/天马/魁/钺/昌/曲/空/劫/鸾/喜/台辅/封诰 等 |
| 杂曜定位 (17 颗) | ✅ 完备 | 单元测试覆盖全部杂曜算法 |
| 长生十二神 | ✅ 完备 | 顺序/起始/排盘(男女)/from 参数 |
| 博士十二神 | ✅ 完备 | 顺序/排盘(阴女顺行) |
| 岁前十二神 | ✅ 完备 | 2023/2025 年 + 大耗/岁破变体 |
| 将前十二神 | ✅ 完备 | 起始位置 + 2023/2025 排布 |
| 运限星(大限/流年) | ✅ 完备 | 12 宫运限星预期分布 + 流年运限星 |
| 运限层(大限/小限/流年/月/日/时) | ✅ 完备 | 索引/柱/十二神/四化/星曜导入 |
| 大限/小限 | ✅ 完备 | 结构/数量/连续/递增/性别对比 |
| 三方四正 | ✅ 完备 | 查询/have_any/have_one_of/is_cast |
| 空宫判断 | ✅ 完备 | r1/r4 含主星宫位 |
| 星曜查找 | ✅ 完备 | 找到/未找到/多宫 |
| 历法转换 | ✅ 完备 | solar↔lunar 双向 + 闰月 |
| 流派配置 | ✅ 完备 | 三合/中州/飞星预设 + Builder + 天地人盘 |
| 农历排盘 | ✅ 完备 | 农历输入与阳历输入一致 |
| 格局识别 | ⚠️ 局部 | 54 格局判定逻辑已具备结构测试；仍缺 54 个格局的独立参考样例逐项验证 |
| 工具函数 | ✅ 完备 | wrap_idx/time_to_index/age_index/坐标转换 |
