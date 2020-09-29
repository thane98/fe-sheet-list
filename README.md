# FE Sheet Lists
Dumps sheet lists/struct metadata from FE9-FE15 bin files.

## Example
**Command**:
```
fe-sheet-list Skill.bin
```

**Output**:
```
スキル
スキル: str
名前: str
ヘルプ: str
タイプ: s8
経験値: u8
消費HP: u8
回数: u8
内射程: u8
アイコン: u8
アイコン大: u8
状態アイコン: u8
攻撃率: u8
防御貫通率: u8
物理ダメージ率: u8
魔法ダメージ率: u8
物間接ダメージ率: u8
地形効果: u8
パラメータ影響率: u8
回復率: u8
威力増減: s16
命中増減: s16
回避増減: s16
必殺増減: s16
物理ダメージ減衰: s16
魔法ダメージ減衰: s16
攻速増減: s16
射程増減: s16
HP+: s8
力+: s8
技+: s8
速さ+: s8
運+: s8
守備+: s8
魔防+: s8
移動+: s8
物反射: u8
魔反射: u8
効果武器: u8
特効: u8
スキルフラグ: u32
アイコンエフェクト: str
チャージエフェクト: str
ヒットエフェクト: str
回避エフェクト: str
カメラ: str
演出: ptr
モーション1: str
モーション2: str
モーション3: str
モーション4: str
モーション5: str

演出
シーケンス: str
```