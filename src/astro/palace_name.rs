//! 十二宫名称定义

/// 十二宫枚举（按固定顺序：从命宫开始顺时针排列）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PalaceName {
    /// 命宫 — 代表个人先天命运、性格与运势的总体格局
    Fate = 0,
    /// 父母宫 — 代表父母、长辈关系及遗传因素
    Parents = 1,
    /// 福德宫 — 代表福气、精神享受与内心世界
    Fortune = 2,
    /// 田宅宫 — 代表不动产、家庭环境与居住状况
    Property = 3,
    /// 官禄宫 — 代表事业、职业发展与社会地位
    Career = 4,
    /// 交友宫 — 代表朋友、同事及人际关系网络
    Friends = 5,
    /// 迁移宫 — 代表外出、旅行及外在环境变迁
    Travel = 6,
    /// 疾厄宫 — 代表身体健康状况与疾病倾向
    Health = 7,
    /// 财帛宫 — 代表财富、财运及理财能力
    Wealth = 8,
    /// 子女宫 — 代表子女状况与亲子关系
    Children = 9,
    /// 夫妻宫 — 代表婚姻、配偶及感情生活
    Spouse = 10,
    /// 兄弟宫 — 代表兄弟姐妹关系及手足缘分
    Siblings = 11,
}

impl_enum_str!(PalaceName, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Fate => ("命宫", "命宮", "Fate", "命宫", "명궁", "Mệnh"),
    Parents => ("父母", "父母", "Parents", "父母", "부모", "Phụ Mẫu"),
    Fortune => ("福德", "福德", "Fortune", "福德", "복덕", "Phúc Đức"),
    Property => ("田宅", "田宅", "Property", "田宅", "전택", "Điền Trạch"),
    Career => ("官禄", "官祿", "Career", "官禄", "관록", "Quan Lộc"),
    Friends => ("交友", "交友", "Friends", "交友", "노복", "Bằng Hữu"),
    Travel => ("迁移", "遷移", "Travel", "迁移", "천이", "Thiên Di"),
    Health => ("疾厄", "疾厄", "Health", "疾厄", "질액", "Tật Ách"),
    Wealth => ("财帛", "财帛", "Wealth", "财帛", "재백", "Tài Bạch"),
    Children => ("子女", "子女", "Children", "子女", "자녀", "Tử Nữ"),
    Spouse => ("夫妻", "夫妻", "Spouse", "夫妻", "부처", "Phu Thê"),
    Siblings => ("兄弟", "兄弟", "Siblings", "兄弟", "형제", "Huynh Đệ"),
});

impl From<usize> for PalaceName {
    fn from(idx: usize) -> Self {
        Self::ALL[idx % 12]
    }
}

impl From<isize> for PalaceName {
    fn from(idx: isize) -> Self {
        Self::from(idx as usize)
    }
}

impl PalaceName {
    /// 获取该宫名的内部索引（0=命宫, 1=父母宫, ... 11=兄弟宫）
    pub fn index(&self) -> usize {
        *self as usize
    }

    /// 十二宫完整列表（从命宫开始）
    pub const ALL: [Self; 12] = [
        Self::Fate,
        Self::Parents,
        Self::Fortune,
        Self::Property,
        Self::Career,
        Self::Friends,
        Self::Travel,
        Self::Health,
        Self::Wealth,
        Self::Children,
        Self::Spouse,
        Self::Siblings,
    ];
}
