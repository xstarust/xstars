//! 星曜名称枚举

use crate::star::StarType;

/// 星曜名称枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum StarName {
    // ========== 主星 (Major) ==========
    /// 紫微星 — 帝王星，代表尊贵与领导力
    Ziwei,
    /// 天机星 — 智慧星，代表思考与谋略
    Tianji,
    /// 太阳星 — 光明星，代表贵显与博爱
    Taiyang,
    /// 武曲星 — 财帛星，代表财富与刚毅
    Wuqu,
    /// 天同星 — 福寿星，代表和顺与福气
    Tiantong,
    /// 廉贞星 — 官禄星，代表清廉与次桃花
    Lianzhen,
    /// 天府星 — 令星，代表安定与库藏
    Tianfu,
    /// 太阴星 — 月亮星，代表温柔与财富
    Taiyin,
    /// 天相星 — 印星，代表公正与辅助
    Tianxiang,
    /// 天梁星 — 荫星，代表长寿与解厄
    Tianliang,
    /// 七杀星 — 杀星，代表威权与肃杀
    Qisha,
    /// 破军星 — 破耗星，代表变革与破坏
    Pojun,
    /// 贪狼星 — 桃花星，代表欲望与才艺
    Tanlang,
    /// 巨门星 — 暗星，代表口舌与是非
    Jumen,
    // ========== 辅星 (Minor) ==========
    /// 左辅星 — 助力星，代表辅助与贵人
    Zuofu,
    /// 右弼星 — 助力星，代表辅助与贵人
    Youbi,
    /// 文昌星 — 文星，代表文学与功名
    Wenchang,
    /// 文曲星 — 文星，代表才艺与口才
    Wenqu,
    /// 禄存星 — 禄星，代表财富与储蓄
    Lucun,
    /// 天马星 — 马星，代表奔波与变动
    Tianma,
    /// 天魁星 — 天乙贵人，代表男性贵人
    Tiankui,
    /// 天钺星 — 天乙贵人，代表女性贵人
    Tianyue,
    /// 天月星 — 星曜，代表健康与疾病
    TianyueSick,
    /// 擎羊星 — 刑星，代表刑伤与冲突
    Qingyang,
    /// 陀罗星 — 忌星，代表拖延与阻碍
    Tuoluo,
    /// 火星 — 火煞星，代表急躁与突发
    Huoxing,
    /// 铃星 — 火煞星，代表惊恐与暗算
    Lingxing,
    /// 地空星 — 空亡星，代表空虚与损耗
    Dikong,
    /// 地劫星 — 劫煞星，代表损失与劫难
    Dijie,
    // ========== 杂曜 (Misc) ==========
    /// 红鸾星 — 桃花星，代表姻缘与喜事
    Hongluan,
    /// 天喜星 — 喜星，代表喜庆与结婚
    Tianxi,
    /// 华盖 — 艺术星，代表才华与孤独（年支三合局）
    Huagai,
    /// 咸池 — 桃花星，代表情欲与浪漫（年支三合局）
    Xianchi,
    /// 天姚星 — 桃花星，代表风流与艳遇
    Tianyao,
    /// 三台星 — 增辉星，代表显贵与成就
    Santai,
    /// 八座星 — 增辉星，代表升迁与地位
    Bazuo,
    /// 恩光星 — 恩宠星，代表恩典与荣耀
    Enguang,
    /// 天贵星 — 贵气星，代表高贵与尊荣
    Tiangui,
    /// 台辅星 — 辅助星，代表助力与辅佐
    Taifu,
    /// 封诰星 — 荣誉星，代表封赏与名声
    Fenggao,
    /// 天官星 — 官禄星，代表官职与权威
    Tianguan,
    /// 天厨星 — 福禄星，代表饮食与福气
    Tianchu,
    /// 天空星 — 空亡星，代表孤独与虚华
    Tiankong,
    /// 阴煞星 — 阴煞星，代表阴邪与小人是非
    Yinsha,
    /// 天德 — 德星，代表吉祥（月系）
    Tiande,
    /// 月德星 — 德星，代表吉祥与化厄
    Yuede,
    /// 天巫星 — 灵气星，代表灵感与宗教缘
    Tianwu,
    /// 天才星 — 才智星，代表智慧与才华
    Tiancai,
    /// 天寿星 — 长寿星，代表长寿与健康
    Tianshou,
    /// 天刑星 — 刑星，代表刑伤与自律
    Tianxing,
    /// 天使星 — 使星，代表灾厄与病痛
    Tianshi,
    /// 天伤星 — 伤星，代表悲伤与损伤
    Tianshang,
    /// 龙池星 — 福星，代表才华与升迁
    Longchi,
    /// 凤阁星 — 文星，代表优雅与艺术
    Fengge,
    /// 天福星 — 福星，代表福气与享受
    TianfuFortune,
    /// 截路星 — 阻碍星，代表阻碍与分离
    Jielu,
    /// 空亡星 — 空亡星，代表空虚与失落
    Kongwang,
    /// 孤辰星 — 孤独星，代表孤僻与独处
    Guchen,
    /// 寡宿星 — 寡宿星，代表孤独与守寡
    Guasu,
    /// 蜚蠊星 — 是非星，代表是非与纠纷
    Feilian,
    /// 破碎星 — 破碎星，代表破败与分离
    Posui,
    /// 天哭星 — 哭星，代表悲伤与哭泣
    Tianku,
    /// 天虚星 — 虚星，代表虚耗与不幸
    Tianxu,
    /// 年解星 — 解神星，代表解除与消灾
    Nianjie,
    /// 旬空星 — 空亡星，代表空虚与落空
    Xunkong,
    /// 解神星 — 解神星，代表解除厄运
    Jieshen,
    /// 截空星 — 空亡星，代表阻碍与落空
    Jiekong,
    // ==================== 神煞 (ShenSha: 长生12 + 博士12 + 岁前12 + 将前12) ====================
    // 长生十二神
    /// 长生 — 万物初生，生命力旺盛的阶段
    ChangSheng,
    /// 沐浴 — 桃花沐浴，感情与欲望的阶段
    MuYu,
    /// 冠带 — 成长冠礼，事业渐入佳境的阶段
    GuanDai,
    /// 临官 — 事业临官，成就巅峰的阶段
    LinGuan,
    /// 帝旺 — 气势最旺，运势顶峰的阶段
    DiWang,
    /// 衰 — 开始衰退，运势渐衰的阶段
    Shuai,
    /// 病 — 衰败多病，运势低迷的阶段
    Bing,
    /// 死 — 死寂停滞，运势冻结的阶段
    Si,
    /// 墓 — 收藏入库，积蓄沉淀的阶段
    Mu,
    /// 绝 — 断绝归寂，旧气消亡的阶段
    Jue,
    /// 胎 — 重新孕育，新气萌生的阶段
    Tai,
    /// 养 — 培养养成，蓄势待发的阶段
    Yang,
    // 博士十二神
    /// 博士 — 代表学识、智慧与文采
    BoShi,
    /// 力士 — 代表力量、权力与威严
    LiShi,
    /// 青龙 — 代表喜事、吉祥与贵人
    QingLong,
    /// 小耗 — 代表小耗损、失财与破费（博士十二神）
    BsXiaoHao,
    /// 将军 — 代表威权、魄力与领导能力
    JiangJun,
    /// 奏书 — 代表文书、奏章与书面事宜
    ZouShu,
    /// 飞廉 — 代表迅速、变动与流动
    FeiLian,
    /// 喜神 — 代表喜庆、欢乐与好事
    XiShen,
    /// 病符 — 代表疾病、灾祸与健康问题（博士十二神）
    BsBingFu,
    /// 大耗 — 耗星，代表破财与损耗（博士十二神）
    BsDaHao,
    /// 伏兵 — 代表暗算、小人是非与突袭
    FuBing,
    /// 官府 — 代表官司、诉讼与官方事务
    GuanFu,
    // 岁前十二神
    /// 岁建 — 代表流年运势的起始与建基
    SuiJian,
    /// 晦气 — 代表不顺、倒霉与运势低落
    HuiQi,
    /// 丧门 — 代表丧事、哀伤与离别
    SangMen,
    /// 贯索 — 代表束缚、牵连与官司纠缠
    GuanSuo,
    /// 官符 — 代表官非、诉讼与公文纠纷
    SqGuanFu,
    /// 小耗 — 代表小耗损、失财与破费（岁前十二神）
    SqXiaoHao,
    /// 大耗 — 耗星，代表破财与损耗（岁前十二神）
    SqDaHao,
    /// 龙德 — 福德星，代表吉祥与德望
    Longde,
    /// 白虎 — 代表血光、凶事与意外伤害
    BaiHu,
    /// 天德（岁前十二神）— 岁前系
    SqTiande,
    /// 吊客 — 代表吊唁、哀伤与悲痛之事
    DiaoKe,
    /// 病符 — 代表疾病、灾祸与健康问题（岁前十二神）
    SqBingFu,
    /// 岁破 — 代表破败、动荡与运势冲克
    SuiPo,
    // 将前十二神
    /// 将星 — 代表领导才能、权威与魄力
    JiangXing,
    /// 攀鞍 — 代表升迁、进步与地位提升
    PanAn,
    /// 岁驿 — 代表变动、奔波与出行
    SuiYi,
    /// 息神 — 代表休息、停滞与休养生息
    XiShenRest,
    /// 华盖（将前十二神）— 将前系
    JqHuagai,
    /// 劫杀 — 劫煞星，代表劫难与竞争
    Jiesha,
    /// 灾煞 — 代表灾难、祸患与意外之灾
    ZaiSha,
    /// 天煞 — 代表天灾、外祸与不可抗力
    TianSha,
    /// 指背 — 代表背后是非、暗箭与议论
    ZhiBei,
    /// 咸池（将前十二神）— 将前系咸池
    JqXianchi,
    /// 月煞 — 代表月令凶煞，不顺与阻碍
    YueSha,
    /// 亡神 — 代表损耗、失亡与精神不振
    WangShen,
    // ========== 运限星 (Yunxian) ==========
    /// 大运天魁星 — 运限层的天魁（十年大运）
    YunKui,
    /// 大运天钺星 — 运限层的天钺（十年大运）
    YunYue,
    /// 大运文昌星 — 运限层的文昌（十年大运）
    YunChang,
    /// 大运文曲星 — 运限层的文曲（十年大运）
    YunQu,
    /// 大运禄存星 — 运限层的禄存（十年大运）
    YunLu,
    /// 大运擎羊星 — 运限层的擎羊（十年大运）
    YunYang,
    /// 大运陀罗星 — 运限层的陀罗（十年大运）
    YunTuo,
    /// 大运天马星 — 运限层的天马（十年大运）
    YunMa,
    /// 大运红鸾星 — 运限层的红鸾（十年大运）
    YunLuan,
    /// 大运天喜星 — 运限层的天喜（十年大运）
    YunXi,
    /// 流年天魁星 — 运限层的天魁（流年）
    LiuKui,
    /// 流年天钺星 — 运限层的天钺（流年）
    LiuYue,
    /// 流年文昌星 — 运限层的文昌（流年）
    LiuChang,
    /// 流年文曲星 — 运限层的文曲（流年）
    LiuQu,
    /// 流年禄存星 — 运限层的禄存（流年）
    LiuLu,
    /// 流年擎羊星 — 运限层的擎羊（流年）
    LiuYang,
    /// 流年陀罗星 — 运限层的陀罗（流年）
    LiuTuo,
    /// 流年天马星 — 运限层的天马（流年）
    LiuMa,
    /// 流年红鸾星 — 运限层的红鸾（流年）
    LiuLuan,
    /// 流年天喜星 — 运限层的天喜（流年）
    LiuXi,
    /// 流月天魁星 — 运限层的天魁（流月）
    YueKui,
    /// 流月天钺星 — 运限层的天钺（流月）
    YueYue,
    /// 流月文昌星 — 运限层的文昌（流月）
    YueChang,
    /// 流月文曲星 — 运限层的文曲（流月）
    YueQu,
    /// 流月禄存星 — 运限层的禄存（流月）
    YueLu,
    /// 流月擎羊星 — 运限层的擎羊（流月）
    YueYang,
    /// 流月陀罗星 — 运限层的陀罗（流月）
    YueTuo,
    /// 流月天马星 — 运限层的天马（流月）
    YueMa,
    /// 流月红鸾星 — 运限层的红鸾（流月）
    YueLuan,
    /// 流月天喜星 — 运限层的天喜（流月）
    YueXi,
    /// 流日天魁星 — 运限层的天魁（流日）
    RiKui,
    /// 流日天钺星 — 运限层的天钺（流日）
    RiYue,
    /// 流日文昌星 — 运限层的文昌（流日）
    RiChang,
    /// 流日文曲星 — 运限层的文曲（流日）
    RiQu,
    /// 流日禄存星 — 运限层的禄存（流日）
    RiLu,
    /// 流日擎羊星 — 运限层的擎羊（流日）
    RiYang,
    /// 流日陀罗星 — 运限层的陀罗（流日）
    RiTuo,
    /// 流日天马星 — 运限层的天马（流日）
    RiMa,
    /// 流日红鸾星 — 运限层的红鸾（流日）
    RiLuan,
    /// 流日天喜星 — 运限层的天喜（流日）
    RiXi,
    /// 流时天魁星 — 运限层的天魁（流时）
    ShiKui,
    /// 流时天钺星 — 运限层的天钺（流时）
    ShiYue,
    /// 流时文昌星 — 运限层的文昌（流时）
    ShiChang,
    /// 流时文曲星 — 运限层的文曲（流时）
    ShiQu,
    /// 流时禄存星 — 运限层的禄存（流时）
    ShiLu,
    /// 流时擎羊星 — 运限层的擎羊（流时）
    ShiYang,
    /// 流时陀罗星 — 运限层的陀罗（流时）
    ShiTuo,
    /// 流时天马星 — 运限层的天马（流时）
    ShiMa,
    /// 流时红鸾星 — 运限层的红鸾（流时）
    ShiLuan,
    /// 流时天喜星 — 运限层的天喜（流时）
    ShiXi,
}

impl_enum_str!(StarName, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    // ========== 主星 (Major) ==========
    Ziwei => ("紫微", "紫微", "Zi Wei", "紫微", "자미", "Tử Vi"),
    Tianji => ("天机", "天機", "Tian Ji", "天机", "천기", "Thiên Cơ"),
    Taiyang => ("太阳", "太陽", "Tai Yang", "太阳", "태양", "Thái Dương"),
    Wuqu => ("武曲", "武曲", "Wu Qu", "武曲", "무곡", "Vũ Khúc"),
    Tiantong => ("天同", "天同", "Tian Tong", "天同", "천동", "Thiên Đồng"),
    Lianzhen => ("廉贞", "廉貞", "Lian Zhen", "廉贞", "염정", "Liêm Trinh"),
    Tianfu => ("天府", "天府", "Tian Fu", "天府", "천부", "Thiên Phủ"),
    Taiyin => ("太阴", "太陰", "Tai Yin", "太阴", "태음", "Thái Âm"),
    Tianxiang => ("天相", "天相", "Tian Xiang", "天相", "천상", "Thiên Tướng"),
    Tianliang => ("天梁", "天梁", "Tian Liang", "天梁", "천량", "Thiên Lương"),
    Qisha => ("七杀", "七殺", "Qi Sha", "七杀", "칠살", "Thất Sát"),
    Pojun => ("破军", "破軍", "Po Jun", "破军", "파군", "Phá Quân"),
    Tanlang => ("贪狼", "貪狼", "Tan Lang", "贪狼", "탐랑", "Tham Lang"),
    Jumen => ("巨门", "巨門", "Ju Men", "巨门", "거문", "Cự Môn"),
    // ========== 辅星 (Minor) ==========
    Zuofu => ("左辅", "左輔", "Zuo Fu", "左辅", "좌보", "Tả Phù"),
    Youbi => ("右弼", "右弼", "You Bi", "右弼", "우필", "Hữu Bật"),
    Wenchang => ("文昌", "文昌", "Wen Chang", "文昌", "문창", "Văn Xương"),
    Wenqu => ("文曲", "文曲", "Wen Qu", "文曲", "문곡", "Văn Khúc"),
    Lucun => ("禄存", "祿存", "Lu Cun", "禄存", "록존", "Lộc Tồn", "lucun", "lucu", "lucn"),
    Tianma => ("天马", "天馬", "Tian Ma", "天马", "천마", "Thiên Mã"),
    Tiankui => ("天魁", "天魁", "Tian Kui", "天魁", "천괴", "Thiên Khôi"),
    Tianyue => ("天钺", "天鉞", "Tian Yue", "天钺", "천월", "Thiên Việt"),
    TianyueSick => ("天月", "天月", "Tian Yue", "天月", "천월", "Thiên Nguyệt"),
    Qingyang => ("擎羊", "擎羊", "Qing Yang", "擎羊", "경양", "Kình Dương"),
    Tuoluo => ("陀罗", "陀羅", "Tuo Luo", "陀罗", "타라", "Đà La", "tuoluo", "tuolu"),
    Huoxing => ("火星", "火星", "Huo Xing", "火星", "화성", "Hỏa Tinh"),
    Lingxing => ("铃星", "鈴星", "Ling Xing", "铃星", "령성", "Linh Tinh"),
    Dikong => ("地空", "地空", "Di Kong", "地空", "지공", "Địa Không"),
    Dijie => ("地劫", "地劫", "Di Jie", "地劫", "지겁", "Địa Kiếp"),
    // ========== 杂曜 (Misc) ==========
    Hongluan => ("红鸾", "紅鸞", "Hong Luan", "红鸾", "홍란", "Hồng Loan"),
    Tianxi => ("天喜", "天喜", "Tian Xi", "天喜", "천희", "Thiên Hỷ"),
    Huagai => ("华盖", "華蓋", "Hua Gai", "华盖", "화개", "Hoa Cái"),
    Xianchi => ("咸池", "咸池", "Xian Chi", "咸池", "함지", "Hàm Trì"),
    Tianyao => ("天姚", "天姚", "Tian Yao", "天姚", "천요", "Thiên Diêu"),
    Santai => ("三台", "三臺", "San Tai", "三台", "삼대", "Tam Thai"),
    Bazuo => ("八座", "八座", "Ba Zuo", "八座", "팔좎", "Bát Tọa"),
    Enguang => ("恩光", "恩光", "En Guang", "恩光", "은광", "Ân Quang"),
    Tiangui => ("天贵", "天貴", "Tian Gui", "天贵", "천귀", "Thiên Quý"),
    Taifu => ("台辅", "臺輔", "Tai Fu", "台辅", "대보", "Thai Phụ"),
    Fenggao => ("封诰", "封誥", "Feng Gao", "封诰", "봉고", "Phong Cáo"),
    Tianguan => ("天官", "天官", "Tian Guan", "天官", "천관", "Thiên Quan"),
    Tianchu => ("天厨", "天廚", "Tian Chu", "天厨", "천주", "Thiên Trù"),
    Tiankong => ("天空", "天空", "Tian Kong", "天空", "천공", "Thiên Không"),
    Yinsha => ("阴煞", "陰煞", "Yin Sha", "阴煞", "음살", "Âm Sát"),
    Tiande => ("天德", "天德", "Tian De", "天德", "천덕", "Thiên Đức"),
    Yuede => ("月德", "月德", "Yue De", "月德", "월덕", "Nguyệt Đức"),
    Tianwu => ("天巫", "天巫", "Tian Wu", "天巫", "천무", "Thiên Vu"),
    Tiancai => ("天才", "天才", "Tian Cai", "天才", "천재", "Thiên Tài"),
    Tianshou => ("天寿", "天壽", "Tian Shou", "天寿", "천수", "Thọ Tinh"),
    Tianxing => ("天刑", "天刑", "Tian Xing", "天刑", "천형", "Thiên Hình"),
    Tianshi => ("天使", "天使", "Tian Shi", "天使", "천사", "Thiên Sứ"),
    Tianshang => ("天伤", "天傷", "Tian Shang", "天伤", "천상", "Thiên Thương"),
    Longchi => ("龙池", "龍池", "Long Chi", "龙池", "용지", "Long Trì"),
    Fengge => ("凤阁", "鳳閣", "Feng Ge", "凤阁", "봉각", "Phượng Các"),
    TianfuFortune => ("天福", "天福", "Tian Fu", "天福", "천복", "Thiên Phúc"),
    Jielu => ("截路", "截路", "Jie Lu", "截路", "절로", "tiệt Lộ"),
    Kongwang => ("空亡", "空亡", "Kong Wang", "空亡", "공망", "Không Vong"),
    Guchen => ("孤辰", "孤辰", "Gu Chen", "孤辰", "고진", "Cô Thần"),
    Guasu => ("寡宿", "寡宿", "Gua Su", "寡宿", "과숙", "Quả Túc"),
    Feilian => ("蜚蠊", "蜚蠊", "Fei Lian", "蜚蠊", "비렴", "Phi Liêm"),
    Posui => ("破碎", "破碎", "Po Sui", "破碎", "파쇄", "phá Toái"),
    Tianku => ("天哭", "天哭", "Tian Ku", "天哭", "천곡", "Thiên Khốc"),
    Tianxu => ("天虚", "天虛", "Tian Xu", "天虚", "천허", "Thiên Hư"),
    Nianjie => ("年解", "年解", "Nian Jie", "年解", "연해", "Niên Giải"),
    Xunkong => ("旬空", "旬空", "Xun Kong", "旬空", "순공", "Tuần Không"),
    Jieshen => ("解神", "解神", "Jie Shen", "解神", "해신", "Giải Thần"),
    Jiekong => ("截空", "截空", "Jie Kong", "截空", "절공", "Tiệt Không"),
    // ==================== 神煞 (ShenSha) ====================
    // 长生十二神
    ChangSheng => ("长生", "長生", "Long Life", "長生", "장생", "Trường Sinh"),
    MuYu => ("沐浴", "沐浴", "Bath", "沐浴", "목욕", "Mục Dục"),
    GuanDai => ("冠带", "冠帶", "Coming of Age", "冠帶", "관대", "Quan Đới"),
    LinGuan => ("临官", "臨官", "Official", "臨官", "임관", "Lâm Quan"),
    DiWang => ("帝旺", "帝旺", "Peak", "帝旺", "제왕", "Đế Vượng"),
    Shuai => ("衰", "衰", "Decline", "衰", "쇠", "Suy"),
    Bing => ("病", "病", "Sick", "病", "병", "Bệnh"),
    Si => ("死", "死", "Death", "死", "사", "Tử"),
    Mu => ("墓", "墓", "Tomb", "墓", "묘", "Mộ"),
    Jue => ("绝", "絕", "End", "絶", "절", "Tuyệt"),
    Tai => ("胎", "胎", "Fetus", "胎", "태", "Thai"),
    Yang => ("养", "養", "Nurture", "養", "양", "Dưỡng"),
    // 博士十二神
    BoShi => ("博士", "博士", "Scholar", "博士", "박사", "Bác Sỹ"),
    LiShi => ("力士", "力士", "Warrior", "力士", "역사", "Lực Sĩ"),
    QingLong => ("青龙", "青龍", "Azure Dragon", "青龍", "청룡", "Thanh Long"),
    BsXiaoHao => ("小耗", "小耗", "Minor Loss", "小耗", "소모", "Tiểu Hao"),
    JiangJun => ("将军", "將軍", "General", "將軍", "장군", "Tướng Quân"),
    ZouShu => ("奏书", "奏書", "Memos", "奏書", "주서", "Tấu Thư"),
    FeiLian => ("飞廉", "飛廉", "Swift Wind", "飛廉", "비렴", "Phi Liêm"),
    XiShen => ("喜神", "喜神", "Joy", "喜神", "희신", "Hỷ Thần"),
    BsBingFu => ("病符", "病符", "Plague", "病符", "병부", "Bệnh Phù"),
    BsDaHao => ("大耗", "大耗", "Da Hao", "大耗", "대호", "Đại Hao"),
    FuBing => ("伏兵", "伏兵", "Ambush", "伏兵", "복병", "Phục Binh"),
    GuanFu => ("官府", "官府", "Authority", "官府", "관부", "Quan Phủ"),
    // 岁前十二神
    SuiJian => ("岁建", "歲建", "Yearly Star", "歲建", "세건", "Tuế Kiến"),
    HuiQi => ("晦气", "晦氣", "Misfortune", "晦氣", "회기", "Hối Khí"),
    SangMen => ("丧门", "喪門", "Death's Door", "喪門", "상문", "Tang Môn"),
    GuanSuo => ("贯索", "貫索", "Shackles", "貫索", "관색", "Quán Tác"),
    SqGuanFu => ("官符", "官符", "Official Seal", "官符", "관부", "Quan Phủ"),
    SqXiaoHao => ("小耗", "小耗", "Minor Loss", "小耗", "소모", "Tiểu Hao"),
    SqDaHao => ("大耗", "大耗", "Da Hao", "大耗", "대호", "Đại Hao"),
    Longde => ("龙德", "龍德", "Long De", "龙德", "용덕", "Long Đức"),
    BaiHu => ("白虎", "白虎", "White Tiger", "白虎", "백호", "Bạch Hổ"),
    SqTiande => ("天德", "天德", "Sq Tian De", "天德", "천덕", "Thiên Đức"),
    DiaoKe => ("吊客", "吊客", "Mourner", "弔客", "도객", "Điếu Khách"),
    SqBingFu => ("病符", "病符", "Plague", "病符", "병부", "Bệnh Phù"),
    SuiPo => ("岁破", "歲破", "Broken Year", "歲破", "세파", "Tuế Phá"),
    // 将前十二神
    JiangXing => ("将星", "將星", "General Star", "將星", "장성", "Tướng Tinh"),
    PanAn => ("攀鞍", "攀鞍", "Climbing Saddle", "攀鞍", "반안", "Bàn Yên"),
    SuiYi => ("岁驿", "歲驛", "Yearly Post", "歲驛", "세역", "Tuế Dịch"),
    XiShenRest => ("息神", "息神", "Rest Spirit", "息神", "식신", "Tức Thần"),
    JqHuagai => ("华盖", "華蓋", "Jq Hua Gai", "华盖", "화개", "Hoa Cái"),
    Jiesha => ("劫杀", "劫殺", "Jie Sha", "劫杀", "겁살", "Kiếp Sát"),
    ZaiSha => ("灾煞", "災煞", "Disaster", "災煞", "재살", "Tai Sát"),
    TianSha => ("天煞", "天煞", "Heavenly Disaster", "天煞", "천살", "Thiên Sát"),
    ZhiBei => ("指背", "指背", "Backstabber", "指背", "지배", "Chỉ Bối"),
    JqXianchi => ("咸池", "咸池", "Jq Xian Chi", "咸池", "함지", "Hàm Trì"),
    YueSha => ("月煞", "月煞", "Lunar Disaster", "月煞", "월살", "Nguyệt Sát"),
    WangShen => ("亡神", "亡神", "Death Spirit", "亡神", "망신", "Vong Thần"),
    // ========== 运限星 (Yunxian) ==========
    YunKui => ("运魁", "運魁", "Yun Kui", "运魁", "천괴(십년)", "Thiên Khôi"),
    YunYue => ("运钺", "運鉞", "Yun Yue", "运钺", "천월(십년)", "Thiên Việt"),
    YunChang => ("运昌", "運昌", "Yun Chang", "运昌", "문창(십년)", "Văn Xương"),
    YunQu => ("运曲", "運曲", "Yun Qu", "运曲", "문곡(십년)", "Văn Khúc"),
    YunLu => ("运禄", "運祿", "Yun Lu", "运禄", "록존(십년)", "Lộc Tồn"),
    YunYang => ("运羊", "運羊", "Yun Yang", "运羊", "경양(십년)", "Kình Dương"),
    YunTuo => ("运陀", "運陀", "Yun Tuo", "运陀", "타라(십년)", "Đà La"),
    YunMa => ("运马", "運馬", "Yun Ma", "运马", "천마(십년)", "Thiên Mã"),
    YunLuan => ("运鸾", "運鸞", "Yun Luan", "运鸾", "홍란(십년)", "Hồng Loan"),
    YunXi => ("运喜", "運喜", "Yun Xi", "运喜", "천희(십년)", "Thiên Hỷ"),
    LiuKui => ("流魁", "流魁", "Liu Kui", "流魁", "천괴(년)", "Lưu Khôi"),
    LiuYue => ("流钺", "流鉞", "Liu Yue", "流钺", "천월(년)", "Lưu Việt"),
    LiuChang => ("流昌", "流昌", "Liu Chang", "流昌", "문창(년)", "Lưu Xương"),
    LiuQu => ("流曲", "流曲", "Liu Qu", "流曲", "문곡(년)", "Lưu Khúc"),
    LiuLu => ("流禄", "流祿", "Liu Lu", "流禄", "록존(년)", "Lưu Lộc"),
    LiuYang => ("流羊", "流羊", "Liu Yang", "流羊", "경양(년)", "Lưu Dương"),
    LiuTuo => ("流陀", "流陀", "Liu Tuo", "流陀", "타라(년)", "Lưu Đà"),
    LiuMa => ("流马", "流馬", "Liu Ma", "流马", "천마(년)", "Lưu Mã"),
    LiuLuan => ("流鸾", "流鸞", "Liu Luan", "流鸾", "홍란(년)", "Lưu Loan"),
    LiuXi => ("流喜", "流喜", "Liu Xi", "流喜", "천희(년)", "Lưu Hỷ"),
    YueKui => ("月魁", "月魁", "Yue Kui", "月魁", "월괴", "Nguyệt Khôi"),
    YueYue => ("月钺", "月鉞", "Yue Yue", "月钺", "월월", "Nguyệt Việt"),
    YueChang => ("月昌", "月昌", "Yue Chang", "月昌", "월창", "Nguyệt Xương"),
    YueQu => ("月曲", "月曲", "Yue Qu", "月曲", "월곡", "Nguyệt Khúc"),
    YueLu => ("月禄", "月祿", "Yue Lu", "月禄", "월록", "Nguyệt Lộc"),
    YueYang => ("月羊", "月羊", "Yue Yang", "月羊", "월양", "Nguyệt Dương"),
    YueTuo => ("月陀", "月陀", "Yue Tuo", "月陀", "월타", "Nguyệt Đà"),
    YueMa => ("月马", "月馬", "Yue Ma", "月马", "월마", "Nguyệt Mã"),
    YueLuan => ("月鸾", "月鸾", "Yue Luan", "月鸾", "월란", "Nguyệt Loan"),
    YueXi => ("月喜", "月喜", "Yue Xi", "月喜", "월희", "Nguyệt Hỷ"),
    RiKui => ("日魁", "日魁", "Ri Kui", "日魁", "일괴", "Nhật Khôi"),
    RiYue => ("日钺", "日鉞", "Ri Yue", "日钺", "일월", "Nhật Việt"),
    RiChang => ("日昌", "日昌", "Ri Chang", "日昌", "일창", "Nhật Xương"),
    RiQu => ("日曲", "日曲", "Ri Qu", "日曲", "일곡", "Nhật Khúc"),
    RiLu => ("日禄", "日祿", "Ri Lu", "日禄", "일록", "Nhật Lộc"),
    RiYang => ("日羊", "日羊", "Ri Yang", "日羊", "일양", "Nhật Dương"),
    RiTuo => ("日陀", "日陀", "Ri Tuo", "日陀", "일타", "Nhật Đà"),
    RiMa => ("日马", "日馬", "Ri Ma", "日马", "일마", "Nhật Mã"),
    RiLuan => ("日鸾", "日鸾", "Ri Luan", "日鸾", "일란", "Nhật Loan"),
    RiXi => ("日喜", "日喜", "Ri Xi", "日喜", "일희", "Nhật Hỷ"),
    ShiKui => ("时魁", "時魁", "Shi Kui", "时魁", "시괴", "Thời Khôi"),
    ShiYue => ("时钺", "時鉞", "Shi Yue", "时钺", "시월", "Thời Việt"),
    ShiChang => ("时昌", "時昌", "Shi Chang", "时昌", "시창", "Thời Xương"),
    ShiQu => ("时曲", "時曲", "Shi Qu", "时曲", "시곡", "Thời Khúc"),
    ShiLu => ("时禄", "時祿", "Shi Lu", "时禄", "시록", "Thời Lộc"),
    ShiYang => ("时羊", "时羊", "Shi Yang", "时羊", "시양", "Thời Dương"),
    ShiTuo => ("时陀", "时陀", "Shi Tuo", "时陀", "시타", "Thời Đà"),
    ShiMa => ("时马", "時馬", "Shi Ma", "时马", "시마", "Thời Mã"),
    ShiLuan => ("时鸾", "時鸾", "Shi Luan", "时鸾", "시란", "Thời Loan"),
    ShiXi => ("时喜", "時喜", "Shi Xi", "时喜", "시희", "Thời Hỷ"),
});

impl StarName {
    /// 获取星曜分类（按安星规则划分）
    pub fn star_type(&self) -> StarType {
        match self {
            // ========== 主星 (Major) ==========
            StarName::Ziwei
            | StarName::Tianji
            | StarName::Taiyang
            | StarName::Wuqu
            | StarName::Tiantong
            | StarName::Lianzhen
            | StarName::Tianfu
            | StarName::Taiyin
            | StarName::Tianxiang
            | StarName::Tianliang
            | StarName::Qisha
            | StarName::Pojun
            | StarName::Tanlang
            | StarName::Jumen => StarType::Major,

            // ========== 辅星 (Minor) ==========
            StarName::Zuofu
            | StarName::Youbi
            | StarName::Wenchang
            | StarName::Wenqu
            | StarName::Lucun
            | StarName::Tianma
            | StarName::Tiankui
            | StarName::Tianyue
            | StarName::Qingyang
            | StarName::Tuoluo
            | StarName::Huoxing
            | StarName::Lingxing
            | StarName::Dikong
            | StarName::Dijie => StarType::Minor,

            // ========== 杂曜 (Misc) ==========
            StarName::TianyueSick
            | StarName::Hongluan
            | StarName::Tianxi
            | StarName::Tianyao
            | StarName::Santai
            | StarName::Bazuo
            | StarName::Enguang
            | StarName::Tiangui
            | StarName::Taifu
            | StarName::Fenggao
            | StarName::Tianguan
            | StarName::Tianchu
            | StarName::Tiankong
            | StarName::Yinsha
            | StarName::Yuede
            | StarName::Tianwu
            | StarName::Tiancai
            | StarName::Tianshou
            | StarName::Tianxing
            | StarName::Tianshi
            | StarName::Tianshang
            | StarName::Longchi
            | StarName::Fengge
            | StarName::TianfuFortune
            | StarName::Jielu
            | StarName::Kongwang
            | StarName::Guchen
            | StarName::Guasu
            | StarName::Feilian
            | StarName::Posui
            | StarName::Tianku
            | StarName::Tianxu
            | StarName::Nianjie
            | StarName::Xunkong
            | StarName::Jieshen
            | StarName::Jiekong
            | StarName::Tiande
            | StarName::Huagai
            | StarName::Xianchi => StarType::Misc,

            // ========== 神煞 (ShenSha) ==========
            StarName::ChangSheng
            | StarName::MuYu
            | StarName::GuanDai
            | StarName::LinGuan
            | StarName::DiWang
            | StarName::Shuai
            | StarName::Bing
            | StarName::Si
            | StarName::Mu
            | StarName::Jue
            | StarName::Tai
            | StarName::Yang
            | StarName::BoShi
            | StarName::LiShi
            | StarName::QingLong
            | StarName::BsXiaoHao
            | StarName::JiangJun
            | StarName::ZouShu
            | StarName::FeiLian
            | StarName::XiShen
            | StarName::BsBingFu
            | StarName::BsDaHao
            | StarName::FuBing
            | StarName::GuanFu
            | StarName::SuiJian
            | StarName::HuiQi
            | StarName::SangMen
            | StarName::GuanSuo
            | StarName::SqGuanFu
            | StarName::SqXiaoHao
            | StarName::SqDaHao
            | StarName::Longde
            | StarName::BaiHu
            | StarName::SqTiande
            | StarName::DiaoKe
            | StarName::SqBingFu
            | StarName::SuiPo
            | StarName::JiangXing
            | StarName::PanAn
            | StarName::SuiYi
            | StarName::XiShenRest
            | StarName::Jiesha
            | StarName::ZaiSha
            | StarName::TianSha
            | StarName::ZhiBei
            | StarName::JqHuagai
            | StarName::JqXianchi
            | StarName::YueSha
            | StarName::WangShen => StarType::ShenSha,

            // ========== 运限星 (Yunxian) ==========
            StarName::YunKui
            | StarName::YunYue
            | StarName::YunChang
            | StarName::YunQu
            | StarName::YunLu
            | StarName::YunYang
            | StarName::YunTuo
            | StarName::YunMa
            | StarName::YunLuan
            | StarName::YunXi
            | StarName::LiuKui
            | StarName::LiuYue
            | StarName::LiuChang
            | StarName::LiuQu
            | StarName::LiuLu
            | StarName::LiuYang
            | StarName::LiuTuo
            | StarName::LiuMa
            | StarName::LiuLuan
            | StarName::LiuXi
            | StarName::YueKui
            | StarName::YueYue
            | StarName::YueChang
            | StarName::YueQu
            | StarName::YueLu
            | StarName::YueYang
            | StarName::YueTuo
            | StarName::YueMa
            | StarName::YueLuan
            | StarName::YueXi
            | StarName::RiKui
            | StarName::RiYue
            | StarName::RiChang
            | StarName::RiQu
            | StarName::RiLu
            | StarName::RiYang
            | StarName::RiTuo
            | StarName::RiMa
            | StarName::RiLuan
            | StarName::RiXi
            | StarName::ShiKui
            | StarName::ShiYue
            | StarName::ShiChang
            | StarName::ShiQu
            | StarName::ShiLu
            | StarName::ShiYang
            | StarName::ShiTuo
            | StarName::ShiMa
            | StarName::ShiLuan
            | StarName::ShiXi => StarType::Yunxian,
        }
    }
}
