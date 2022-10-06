use phf::phf_map;

pub const WHITE: u8 = 0xf1;
pub const RED: u8 = 0xf2;
pub const MUGSHOT: u8 = 0xf3;
pub const ANSWER: u8 = 0xf4;
pub const TOP: u8 = 0xfa;
pub const BOTTOM: u8 = 0xfb;
pub const LF: u8 = 0xfc;
pub const NEXT: u8 = 0xfd;
pub const RETURN: u8 = 0xfe;

pub const FACTORY: [u8; 3] = [0xF0, 0x45, 0xEE];
pub const OCCASION: [u8; 3] = [0xEE, 0xF0, 0xA8];

pub static MUGSHOTS: phf::Map<&'static str, u8> = phf_map! {
  "INLINE" => 0x00,
  "NO_MUGSHOT" => 0x01,
  "ゼロ"=> 0x02,
  "ブレイジン・フリザード"=> 0x03,
  "チルドレ・イナラビッタ"=> 0x04,
  "ヘルバット・シルト"=> 0x05,
  "キュービット・フォクスター"=> 0x07,
  "グラチャー・レ・カクタンク"=> 0x08,
  "トレテスタ・ケルベリアン"=> 0x0A,
  "ブレイジン・フリザード(人間)"=> 0x0B,
  "チルドレ・イナラビッタ(人間)"=> 0x0C,
  "ヘルバット・シルト(人間)"=> 0x0D,
  "キュービット・フォクスター(人間)"=> 0x0F,
  "グラチャー・レ・カクタンク(人間)"=> 0x10,
  "アヌビステップ"=> 0x13,
  "ハヌマシーンR"=> 0x14,
  "ブリザックスタグロフR"=> 0x15,
  "X"=> 0x1A,
  "コピーX"=> 0x1B,
  "マザーエルフ"=> 0x1C,
  "ドクター・バイル(1)"=> 0x1D,
  "シエル"=> 0x20,
  "セルヴォ"=> 0x21,
  "ドクター・バイル(2)"=> 0x29,
  "ジョーヌ"=> 0x30,
  "ルージュ"=> 0x31,
  "SOUND_ONLY"=> 0x32,
  "SOUND_ONLY2"=> 0x35,
  "コルボー"=> 0x36,
  "コピーX2"=> 0x37,
};

pub static CHARMAP: phf::Map<&'static str, u16> = phf_map! {
  " "=> 0x00,
  "0"=> 0x01,
  "1"=> 0x02,
  "2"=> 0x03,
  "3"=> 0x04,
  "4"=> 0x05,
  "5"=> 0x06,
  "6"=> 0x07,
  "7"=> 0x08,
  "8"=> 0x09,
  "9"=> 0x0A,
  "A"=> 0x0B,
  "B"=> 0x0C,
  "C"=> 0x0D,
  "D"=> 0x0E,
  "E"=> 0x0F,
  "F"=> 0x10,
  "G"=> 0x11,
  "H"=> 0x12,
  "I"=> 0x13,
  "J"=> 0x14,
  "K"=> 0x15,
  "L"=> 0x16,
  "M"=> 0x17,
  "N"=> 0x18,
  "O"=> 0x19,
  "P"=> 0x1A,
  "Q"=> 0x1B,
  "R"=> 0x1C,
  "S"=> 0x1D,
  "T"=> 0x1E,
  "U"=> 0x1F,
  "V"=> 0x20,
  "W"=> 0x21,
  "X"=> 0x22,
  "Y"=> 0x23,
  "Z"=> 0x24,
  "ぁ"=> 0x25,
  "あ"=> 0x26,
  "ぃ"=> 0x27,
  "い"=> 0x28,
  "ぅ"=> 0x29,
  "う"=> 0x2A,
  "ぇ"=> 0x2B,
  "え"=> 0x2C,
  "ぉ"=> 0x2D,
  "お"=> 0x2E,
  "か"=> 0x2F,
  "が"=> 0x30,
  "き"=> 0x31,
  "ぎ"=> 0x32,
  "く"=> 0x33,
  "ぐ"=> 0x34,
  "け"=> 0x35,
  "げ"=> 0x36,
  "こ"=> 0x37,
  "ご"=> 0x38,
  "さ"=> 0x39,
  "ざ"=> 0x3A,
  "し"=> 0x3B,
  "じ"=> 0x3C,
  "す"=> 0x3D,
  "ず"=> 0x3E,
  "せ"=> 0x3F,
  "ぜ"=> 0x40,
  "そ"=> 0x41,
  "ぞ"=> 0x42,
  "た"=> 0x43,
  "だ"=> 0x44,
  "ち"=> 0x45,
  "ぢ"=> 0x46,
  "っ"=> 0x47,
  "つ"=> 0x48,
  "づ"=> 0x49,
  "て"=> 0x4A,
  "で"=> 0x4B,
  "と"=> 0x4C,
  "ど"=> 0x4D,
  "な"=> 0x4E,
  "に"=> 0x4F,
  "ぬ"=> 0x50,
  "ね"=> 0x51,
  "の"=> 0x52,
  "は"=> 0x53,
  "ば"=> 0x54,
  "ぱ"=> 0x55,
  "ひ"=> 0x56,
  "び"=> 0x57,
  "ぴ"=> 0x58,
  "ふ"=> 0x59,
  "ぶ"=> 0x5A,
  "ぷ"=> 0x5B,
  "へ"=> 0x5C,
  "べ"=> 0x5D,
  "ぺ"=> 0x5E,
  "ほ"=> 0x5F,
  "ぼ"=> 0x60,
  "ぽ"=> 0x61,
  "ま"=> 0x62,
  "み"=> 0x63,
  "む"=> 0x64,
  "め"=> 0x65,
  "も"=> 0x66,
  "ゃ"=> 0x67,
  "や"=> 0x68,
  "ゅ"=> 0x69,
  "ゆ"=> 0x6A,
  "ょ"=> 0x6B,
  "よ"=> 0x6C,
  "ら"=> 0x6D,
  "り"=> 0x6E,
  "る"=> 0x6F,
  "れ"=> 0x70,
  "ろ"=> 0x71,
  "ゎ"=> 0x72,
  "わ"=> 0x73,
  "を"=> 0x74,
  "ん"=> 0x75,
  "ァ"=> 0x76,
  "ア"=> 0x77,
  "ィ"=> 0x78,
  "イ"=> 0x79,
  "ゥ"=> 0x7A,
  "ウ"=> 0x7B,
  "ェ"=> 0x7C,
  "エ"=> 0x7D,
  "ォ"=> 0x7E,
  "オ"=> 0x7F,
  "カ"=> 0x80,
  "ガ"=> 0x81,
  "キ"=> 0x82,
  "ギ"=> 0x83,
  "ク"=> 0x84,
  "グ"=> 0x85,
  "ケ"=> 0x86,
  "ゲ"=> 0x87,
  "コ"=> 0x88,
  "ゴ"=> 0x89,
  "サ"=> 0x8A,
  "ザ"=> 0x8B,
  "シ"=> 0x8C,
  "ジ"=> 0x8D,
  "ス"=> 0x8E,
  "ズ"=> 0x8F,
  "セ"=> 0x90,
  "ゼ"=> 0x91,
  "ソ"=> 0x92,
  "ゾ"=> 0x93,
  "タ"=> 0x94,
  "ダ"=> 0x95,
  "チ"=> 0x96,
  "ッ"=> 0x98,
  "ツ"=> 0x99,
  "テ"=> 0x9B,
  "デ"=> 0x9C,
  "ト"=> 0x9D,
  "ド"=> 0x9E,
  "ナ"=> 0x9F,
  "ニ"=> 0xA0,
  "ヌ"=> 0xA1,
  "ネ"=> 0xA2,
  "ノ"=> 0xA3,
  "ハ"=> 0xA4,
  "バ"=> 0xA5,
  "パ"=> 0xA6,
  "ヒ"=> 0xA7,
  "ビ"=> 0xA8,
  "ピ"=> 0xA9,
  "フ"=> 0xAA,
  "ブ"=> 0xAB,
  "プ"=> 0xAC,
  "ヘ"=> 0xAD,
  "ベ"=> 0xAE,
  "ペ"=> 0xAF,
  "ホ"=> 0xB0,
  "ボ"=> 0xB1,
  "ポ"=> 0xB2,
  "マ"=> 0xB3,
  "ミ"=> 0xB4,
  "ム"=> 0xB5,
  "メ"=> 0xB6,
  "モ"=> 0xB7,
  "ャ"=> 0xB8,
  "ヤ"=> 0xB9,
  "ュ"=> 0xBA,
  "ユ"=> 0xBB,
  "ョ"=> 0xBC,
  "ヨ"=> 0xBD,
  "ラ"=> 0xBE,
  "リ"=> 0xBF,
  "ル"=> 0xC0,
  "レ"=> 0xC1,
  "ロ"=> 0xC2,
  "ヮ"=> 0xC3,
  "ワ"=> 0xC4,
  "ヲ"=> 0xC5,
  "ン"=> 0xC6,
  "ヴ"=> 0xC7,
  "ヵ"=> 0xC8,
  "ヶ"=> 0xC9,
  "！"=> 0xCA,
  "↿"=> 0xCB,
  "↾"=> 0xCC,
  "▷"=> 0xCD,
  "%"=> 0xCE,
  "&"=> 0xCF,
  "'"=> 0xD0,
  "("=> 0xD1,
  ")"=> 0xD2,
  "*"=> 0xD3,
  "+"=> 0xD4, // PLUS
  "、"=> 0xD5,
  "-"=> 0xD6,
  "。"=> 0xD7,
  "/"=> 0xD8,
  ":"=> 0xD9,
  "⊣"=> 0xDA,
  "="=> 0xDB,
  "⊢"=> 0xDC,
  "？"=> 0xDD,
  "["=> 0xDE,
  "]"=> 0xDF,
  "_"=> 0xE0,
  "〜"=> 0xE1,
  "「"=> 0xE2,
  "」"=> 0xE3,
  ".."=> 0xE4,
  "."=> 0xE5,
  "★"=> 0xE6,
  "☆"=> 0xE7,
  "\""=> 0xE9,
  "・"=> 0xEA,
  "Σ"=> 0xEB,
  "ー"=> 0xEC,
  "▼"=> 0xFD,
  "✚" => 0xDADC, // 十字ボタン
  "思"=> 0xF000,
  "気"=> 0xF001,
  "力"=> 0xF002,
  "人"=> 0xF003,
  "間"=> 0xF004,
  "機"=> 0xF005,
  "械"=> 0xF006,
  "本"=> 0xF007,
  "当"=> 0xF008,
  "年"=> 0xF009,
  "月"=> 0xF00A,
  "日"=> 0xF00B,
  "伝"=> 0xF00C,
  "説"=> 0xF00D,
  "全"=> 0xF00E,
  "化"=> 0xF00F,
  "自"=> 0xF010,
  "平"=> 0xF012,
  "和"=> 0xF013,
  "不"=> 0xF016,
  "安"=> 0xF017,
  "聞"=> 0xF018,
  "言"=> 0xF019,
  "知"=> 0xF01A,
  "理"=> 0xF01B,
  "新"=> 0xF01D,
  "古"=> 0xF01E,
  "旧"=> 0xF01F,
  "他"=> 0xF020,
  "最"=> 0xF021,
  "近"=> 0xF022,
  "遠"=> 0xF023,
  "方"=> 0xF024,
  "助"=> 0xF025,
  "未"=> 0xF026,
  "来"=> 0xF027,
  "過"=> 0xF028,
  "去"=> 0xF029,
  "生"=> 0xF02A,
  "死"=> 0xF02B,
  "科"=> 0xF02C,
  "学"=> 0xF02D,
  "同"=> 0xF02E,
  "点"=> 0xF02F,
  "口"=> 0xF030,
  "目"=> 0xF031,
  "大"=> 0xF032,
  "小"=> 0xF033,
  "感"=> 0xF034,
  "地"=> 0xF035,
  "終"=> 0xF036,
  "動"=> 0xF038,
  "止"=> 0xF039,
  "右"=> 0xF03B,
  "左"=> 0xF03C,
  "上"=> 0xF03D,
  "下"=> 0xF03E,
  "時"=> 0xF03F,
  "転"=> 0xF043,
  "送"=> 0xF044,
  "工"=> 0xF045,
  "信"=> 0xF046,
  "高"=> 0xF047,
  "多"=> 0xF049,
  "少"=> 0xF04A,
  "真"=> 0xF04B,
  "回"=> 0xF04C,
  "在"=> 0xF04D,
  "軍"=> 0xF04E,
  "深"=> 0xF04F,
  "出"=> 0xF050,
  "入"=> 0xF051,
  "声"=> 0xF052,
  "型"=> 0xF054,
  "砂"=> 0xF055,
  "発"=> 0xF057,
  "前"=> 0xF058,
  "後"=> 0xF059,
  "守"=> 0xF05A,
  "現"=> 0xF05B,
  "会"=> 0xF05C,
  "基"=> 0xF05D,
  "作"=> 0xF05F,
  "団"=> 0xF061,
  "事"=> 0xF062,
  "無"=> 0xF063,
  "神"=> 0xF064,
  "世"=> 0xF068,
  "戦"=> 0xF069,
  "争"=> 0xF06A,
  "手"=> 0xF06B,
  "実"=> 0xF06C,
  "名"=> 0xF06D,
  "四"=> 0xF070,
  "天"=> 0xF071,
  "王"=> 0xF072,
  "赤"=> 0xF073,
  "青"=> 0xF074,
  "緑"=> 0xF075,
  "者"=> 0xF076,
  "内"=> 0xF077,
  "外"=> 0xF078,
  "強"=> 0xF079,
  "弱"=> 0xF07A,
  "使"=> 0xF07C,
  "用"=> 0xF07D,
  "悪"=> 0xF07E,
  "何"=> 0xF07F,
  "呼"=> 0xF081,
  "以"=> 0xF082,
  "再"=> 0xF083,
  "々"=> 0xF084,
  "見"=> 0xF086,
  "消"=> 0xF087,
  "女"=> 0xF08A,
  "行"=> 0xF08B,
  "分"=> 0xF08C,
  "部"=> 0xF08D,
  "形"=> 0xF08E,
  "話"=> 0xF08F,
  "体"=> 0xF090,
  "倍"=> 0xF091,
  "巨"=> 0xF092,
  "界"=> 0xF095,
  "永"=> 0xF096,
  "仲"=> 0xF097,
  "中"=> 0xF098,
  "隊"=> 0xF099,
  "心"=> 0xF09B,
  "明"=> 0xF09C,
  "集"=> 0xF09D,
  "利"=> 0xF09E,
  "向"=> 0xF09F,
  "賢"=> 0xF0A0,
  "闘"=> 0xF0A1,
  "妖"=> 0xF0A2,
  "隠"=> 0xF0A3,
  "将"=> 0xF0A4,
  "今"=> 0xF0A5,
  "系"=> 0xF0A6,
  "的"=> 0xF0A7,
  "足"=> 0xF0A9,
  "通"=> 0xF0AA,
  "開"=> 0xF0AD,
  "改"=> 0xF0AE,
  "造"=> 0xF0AF,
};