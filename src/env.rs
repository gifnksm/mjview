use crate::{
    agari_type::AgariType, hai::Hai, hai_category::HaiCategory, jun_tehai::JunTehai, tehai::Tehai,
};
use enumflags2::{bitflags, BitFlags};
use js_sys::Array;
use std::{collections::HashMap, str::FromStr};
use wasm_bindgen::prelude::*;

#[bitflags]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Tehai,
    Richi,
    Ippatsu,
    Rinshan,
    Haitei,
    Tenho,
    Bakaze,
    Jikaze,
    Dora,
    Uradora,
    Aotenjo,
}

impl Item {
    fn as_str(&self) -> &'static str {
        use Item::*;
        match self {
            Tehai => "tehai",
            Richi => "richi",
            Ippatsu => "ippatsu",
            Rinshan => "rinshan",
            Haitei => "haitei",
            Tenho => "tenho",
            Bakaze => "bakaze",
            Jikaze => "jikaze",
            Dora => "dora",
            Uradora => "uradora",
            Aotenjo => "aotenjo",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RichiType {
    /// 立直
    Richi,
    /// ダブル立直
    Daburi,
}

impl RichiType {
    fn as_str(&self) -> &'static str {
        match self {
            RichiType::Richi => "richi",
            RichiType::Daburi => "daburi",
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Env {
    pub(crate) richi: Option<RichiType>,
    pub(crate) ippatsu: bool,
    pub(crate) rinshan: bool,
    pub(crate) haitei: bool,
    pub(crate) tenho: bool,
    pub(crate) bakaze: Hai,
    pub(crate) jikaze: Hai,
    pub(crate) dora: Vec<Hai>,
    pub(crate) uradora: Vec<Hai>,
    pub(crate) aotenjo: bool,
}

#[derive(Debug, Clone, Default)]
struct HaiCount {
    all: HashMap<(HaiCategory, u8), (BitFlags<Item>, usize)>,
    tehai: HashMap<(HaiCategory, u8), usize>,
    dora: HashMap<(HaiCategory, u8), usize>,
    uradora: HashMap<(HaiCategory, u8), usize>,
}

impl Env {
    #[cfg(test)]
    pub(crate) fn new_empty(bakaze: Hai, jikaze: Hai) -> Env {
        Env {
            richi: None,
            ippatsu: false,
            rinshan: false,
            haitei: false,
            tenho: false,
            bakaze,
            jikaze,
            dora: vec![],
            uradora: vec![],
            aotenjo: false,
        }
    }

    fn check_props(&self, tehai: Option<&Tehai>) -> Vec<(BitFlags<Item>, String)> {
        use {AgariType::*, Item::*, RichiType as RT};

        let mut res = vec![];

        let any_richi = self.richi.is_some();
        let agari_type = tehai.map(|tehai| tehai.agari_hai().type_());
        let agari_hai = tehai.map(|tehai| tehai.agari_hai().hai());
        let menzen = tehai.map(|tehai| tehai.is_menzen()).unwrap_or_default();
        let oya = self.bakaze.is_same(&self.jikaze);
        let has_furo = tehai
            .map(|tehai| !tehai.furo().is_empty())
            .unwrap_or_default();

        let haitei_name = match agari_type {
            Some(Tsumo) => "海底",
            Some(Ron) => "河底",
            None => "海底/河底",
        };
        let tenho_name = if oya { "天和" } else { "地和" };
        let rinshan_name = match agari_type {
            Some(Tsumo) => "嶺上開花",
            Some(Ron) => "搶槓",
            None => "嶺上開花/搶槓",
        };
        let richi_name = match self.richi {
            Some(RT::Richi) => "立直",
            Some(RT::Daburi) => "ダブル立直",
            None => "立直/ダブル立直",
        };

        let hai_count = self.check_hai_count(tehai, &mut res);

        if self.ippatsu {
            let item = Ippatsu;
            if !any_richi {
                res.push((item | Richi, "一発は立直/ダブル立直時のみ成立します".into()));
            }
            if self.rinshan && agari_type == Some(Tsumo) {
                res.push((item | Rinshan, "一発と嶺上開花は複合しません".into()));
            }
        }
        if self.richi.is_some() && !menzen {
            res.push((Tehai | Richi, format!("{}は門前時のみ可能です", richi_name)));
        }
        if self.richi == Some(RT::Daburi) && self.ippatsu {
            let item = Richi | Ippatsu;
            if self.haitei {
                res.push((
                    item | Haitei,
                    format!("ダブル立直の一発と{}は複合しません", haitei_name),
                ));
            }
            if self.dora.len() > 1 {
                res.push((
                    item | Dora,
                    "ダブル立直の一発の場合ドラ表示牌は必ず1枚です".into(),
                ))
            }
        }
        if self.tenho {
            let item = Tenho;
            if agari_type == Some(Ron) {
                res.push((
                    item | Tehai,
                    format!("{}はツモあがりの場合のみ成立します", tenho_name),
                ));
            }
            if has_furo {
                res.push((
                    item | Tehai,
                    format!("副露がある場合{}にはなりません", tenho_name),
                ));
            }
            if self.richi.is_some() {
                res.push((
                    item | Richi,
                    format!("{}と{}は複合しません", tenho_name, richi_name),
                ));
            }
            if self.rinshan {
                res.push((
                    item | Rinshan,
                    format!("{}と{}は複合しません", tenho_name, rinshan_name),
                ));
            }
            if self.haitei {
                res.push((
                    item | Rinshan,
                    format!("{}と{}は複合しません", tenho_name, haitei_name),
                ));
            }
        }
        if self.rinshan && agari_type == Some(Ron) {
            let agari_hai = agari_hai.unwrap();
            let agari_hai = (agari_hai.category(), agari_hai.number());
            if *hai_count.tehai.get(&agari_hai).unwrap_or(&0) > 1 {
                res.push((
                    Tehai.into(),
                    "搶槓のあがり牌が純手牌/副露に含まれています".into(),
                ));
            }
            if *hai_count.dora.get(&agari_hai).unwrap_or(&0) > 0 {
                res.push((
                    Dora.into(),
                    "搶槓のあがり牌がドラ表示牌に含まれています".into(),
                ));
            }
            if *hai_count.uradora.get(&agari_hai).unwrap_or(&0) > 0 {
                res.push((
                    Uradora.into(),
                    "搶槓のあがり牌が裏ドラ表示牌に含まれています".into(),
                ));
            }
        }
        if self.dora.is_empty() {
            res.push((Dora.into(), "ドラ表示牌が0枚です".into()));
        } else if self.dora.len() > 5 {
            res.push((
                Dora.into(),
                format!("ドラ表示牌が6枚以上あります ({}枚)", self.dora.len()),
            ));
        }
        if any_richi {
            if self.dora.len() != self.uradora.len() {
                res.push((
                    Dora | Uradora,
                    format!(
                        "ドラ表示牌 ({}枚) と裏ドラ表示牌 ({}枚) の枚数が異なります",
                        self.dora.len(),
                        self.uradora.len()
                    ),
                ));
            }
            if self.uradora.is_empty() {
                res.push((
                    Uradora.into(),
                    format!("{}していますが裏ドラ表示牌が0枚です", richi_name),
                ));
            } else if self.uradora.len() > 5 {
                res.push((
                    Uradora.into(),
                    format!("裏ドラ表時牌が6枚以上あります ({}枚)", self.uradora.len()),
                ));
            }
        } else if !self.uradora.is_empty() {
            res.push((
                Uradora.into(),
                "裏ドラが有効なのは立直/ダブル立直時のみです".into(),
            ));
        }
        res
    }

    fn check_hai_count(
        &self,
        tehai: Option<&Tehai>,
        res: &mut Vec<(BitFlags<Item>, String)>,
    ) -> HaiCount {
        use Item::*;

        let mut count = HaiCount::default();
        if let Some(tehai) = tehai {
            for hai in tehai.all_hai() {
                let hai = (hai.category(), hai.number());
                *count.tehai.entry(hai).or_default() += 1;
                let (item, count) = count.all.entry(hai).or_default();
                *item |= Tehai;
                *count += 1;
            }
        }
        for hai in &self.dora {
            let hai = (hai.category(), hai.number());
            *count.dora.entry(hai).or_default() += 1;
            let (item, count) = count.all.entry(hai).or_default();
            *item |= Dora;
            *count += 1;
        }
        for hai in &self.uradora {
            let hai = (hai.category(), hai.number());
            *count.uradora.entry(hai).or_default() += 1;
            let (item, count) = count.all.entry(hai).or_default();
            *item |= Uradora;
            *count += 1;
        }

        for (item, map) in &[
            (Tehai, &count.tehai),
            (Dora, &count.dora),
            (Uradora, &count.uradora),
        ] {
            for (key, value) in *map {
                if *value == 0 {
                    continue;
                }
                if *value > 4 {
                    res.push((
                        (*item).into(),
                        format!("`{}{}` が5枚以上あります ({}枚)", key.1, key.0, *value),
                    ));
                    continue;
                }
                let (items, all_count) = count.all[key];
                if all_count > 4 {
                    let name = items
                        .iter()
                        .map(|item| match item {
                            Tehai => "手牌",
                            Dora => "ドラ表示牌",
                            Uradora => "裏ドラ表示牌",
                            _ => unreachable!(),
                        })
                        .fold(String::new(), |mut out, item| {
                            if !out.is_empty() {
                                out.push('、');
                            }
                            out.push_str(item);
                            out
                        });
                    res.push((
                        (*item).into(),
                        format!(
                            "{}合わせて `{}{}` が5枚以上あります ({}枚)",
                            name, key.1, key.0, all_count
                        ),
                    ));
                }
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Env {
    #[wasm_bindgen(constructor)]
    pub fn new_js() -> Env {
        Env {
            richi: None,
            ippatsu: false,
            rinshan: false,
            haitei: false,
            tenho: false,
            bakaze: Hai::from_str("1j").unwrap(),
            jikaze: Hai::from_str("1j").unwrap(),
            dora: vec![],
            uradora: vec![],
            aotenjo: false,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn richi(&self) -> String {
        self.richi
            .map(|r| r.as_str().to_string())
            .unwrap_or_default()
    }

    #[wasm_bindgen(setter)]
    pub fn set_richi(&mut self, value: &str) {
        self.richi = match value {
            "richi" => Some(RichiType::Richi),
            "daburi" => Some(RichiType::Daburi),
            "" => None,
            _ => panic!("Invalid richi type str: {}", value),
        };
    }

    #[wasm_bindgen(getter)]
    pub fn ippatsu(&self) -> bool {
        self.ippatsu
    }

    #[wasm_bindgen(setter)]
    pub fn set_ippatsu(&mut self, value: bool) {
        self.ippatsu = value;
    }

    #[wasm_bindgen(getter)]
    pub fn rinshan(&self) -> bool {
        self.rinshan
    }

    #[wasm_bindgen(setter)]
    pub fn set_rinshan(&mut self, value: bool) {
        self.rinshan = value;
    }

    #[wasm_bindgen(getter)]
    pub fn haitei(&self) -> bool {
        self.haitei
    }

    #[wasm_bindgen(setter)]
    pub fn set_haitei(&mut self, value: bool) {
        self.haitei = value;
    }

    #[wasm_bindgen(getter)]
    pub fn tenho(&self) -> bool {
        self.tenho
    }

    #[wasm_bindgen(setter)]
    pub fn set_tenho(&mut self, value: bool) {
        self.tenho = value;
    }

    #[wasm_bindgen(getter)]
    pub fn bakaze(&self) -> Hai {
        self.bakaze
    }

    #[wasm_bindgen(setter = bakaze)]
    pub fn set_bakaze_js(&mut self, value: &Hai) {
        self.bakaze = *value;
    }

    #[wasm_bindgen(getter)]
    pub fn jikaze(&self) -> Hai {
        self.jikaze
    }

    #[wasm_bindgen(setter = jikaze)]
    pub fn set_jikaze_js(&mut self, value: &Hai) {
        self.jikaze = *value;
    }

    #[wasm_bindgen(js_name = "setDora")]
    pub fn set_dora_js(&mut self, value: String) -> Result<(), JsValue> {
        let tehai = JunTehai::from_str(&value).map_err(|e| e.to_string())?;
        self.dora = tehai.as_slice().into();
        Ok(())
    }

    #[wasm_bindgen(getter = dora)]
    pub fn dora_js(&self) -> Box<[JsValue]> {
        self.dora.iter().copied().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = "setUradora")]
    pub fn set_uradora_js(&mut self, value: String) -> Result<(), JsValue> {
        let tehai = JunTehai::from_str(&value).map_err(|e| e.to_string())?;
        self.uradora = tehai.as_slice().into();
        Ok(())
    }

    #[wasm_bindgen(getter = uradora)]
    pub fn uradora_js(&self) -> Box<[JsValue]> {
        self.uradora.iter().copied().map(JsValue::from).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn aotenjo(&self) -> bool {
        self.aotenjo
    }

    #[wasm_bindgen(setter)]
    pub fn set_aotenjo(&mut self, value: bool) {
        self.aotenjo = value;
    }

    fn check_props_common_js(&self, tehai: Option<&Tehai>) -> Box<[JsValue]> {
        self.check_props(tehai)
            .into_iter()
            .map(|(flags, message)| {
                JsValue::from(Array::of2(
                    &flags
                        .iter()
                        .map(|item| JsValue::from(item.as_str()))
                        .collect::<Array>(),
                    &JsValue::from(message),
                ))
            })
            .collect()
    }

    #[wasm_bindgen(js_name = checkPropsWithTehai)]
    pub fn check_props_with_tehai_js(&self, tehai: &Tehai) -> Box<[JsValue]> {
        self.check_props_common_js(Some(tehai))
    }

    #[wasm_bindgen(js_name = checkPropsWithoutTehai)]
    pub fn check_props_without_tehai_js(&self) -> Box<[JsValue]> {
        self.check_props_common_js(None)
    }
}
