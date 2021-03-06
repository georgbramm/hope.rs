//! This is the documentation for the `AhOPE` scheme:
//!
//! * Developped by
//! * Published in
//! * Available from
//! * Type encryption (order preserving)
//! * Setting bilinear groups (asymmetric)
//! * Authors Georg Bramm
//! * Date: 04/2018
//!
//! # Examples
//!
//! ```
//!use rabe::schemes::hOPE::*;
//!let (_pk, _msk) = setup();
//! ```
extern crate bn;
extern crate serde;
#[cfg(feature = "serde_derive")]
extern crate serde_derive;
extern crate serde_json;
extern crate mongodb;

use ::hope::protocol::model::*;
use crate::bplus::Tree;
use crate::websocket::HopeWebSocket;
use crate::paillier::Paillier;
use bn::*;
use std::string::String;
use crate::paillier::*;
use std::cmp::Ordering;
use std::ops::AddAssign;
use std::ops::Sub as StdSub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::fs::File;
use std::error::Error;
use std::io::{Read, Write};
use std::path::Path;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use mongodb::oid::ObjectId;
use serde_derive::{Serialize, Deserialize};
use actix_web_actors::ws;
use num_bigint::BigInt;

const DEGREE: usize = 4;
 
/// ehOPE scheme
pub struct hope<'a> {
    /// HopeWebsocket communication
    pub _ws: &'a HopeWebSocket,
    /// ehOPE System Parameters
    pub _sp: hopeSP,
    /// code tree of hOPE scheme
    pub _tree: Tree,
    /// lookup table of hOPE scheme
    pub _apl: BTreeMap<Vec<u8>, ObjectId>,
    /// Optional keypair
    pub _key: Option<hopeK>,
}

impl hope<'_> {
    pub fn new(_name: String, _ws: &HopeWebSocket) -> hope {
        // return System
        hope {
            _ws: _ws,
            _sp: hopeSP::new(_name),
            _tree: Tree::new(DEGREE),
            _apl: BTreeMap::new(),
            _key: hope::keygen(),
        }
    }
    
    pub fn from_sp(_sp: hopeSP, _ws: &HopeWebSocket) -> hope {
        // return System
        hope {
            _ws: _ws,
            _sp: _sp,
            _tree: Tree::new(DEGREE),
            _apl: BTreeMap::new(),
            _key: hope::keygen(),
        }
    }    

    pub fn keygen() -> Option<hopeK> {
    	let (ek, dk) = Paillier::keygen(256);
        Some(hopeK::new(ek,dk))
    } 

    pub fn parameters(&self) -> hopeSP {
    	self._sp.clone()
    }

    pub fn encrypt(&mut self, _m: BigInt) -> Option<hopeCT> {
        if let Some(_ek) = self.enc_key() {
            self.encrypt_ek(&_ek, _m);
        }
        None
    }

    pub fn encrypt_ek(&mut self, _ek: &PaillierEncryptionKey, _m: BigInt) -> Option<hopeCT> {
        let _c = Paillier::encrypt(&_ek, &_m);
        // return pk_u and sk_u
        match Fr::from_str(&_m.to_string()) {
            Some(_fr) => {
                let _g = self._sp._p.mul(_fr);
                match self.lookup_apl(_g) {
                    Some(_ct) => return Some(_ct),
                    None => {
                        let _h = pairing(_g, self._sp._q);
                        let _id = ObjectId::new().unwrap();
                        let leaf = hopeLeaf::new(_id.clone(), _c.clone(), 0);
                        self.insert_tree(leaf);
                        self.update_tree();
                        match self.lookup_tree(_id.clone()) {
                            Some(_code) => {
                                let _hct = hopeCT::from_id(_id, _c, _g, _h, _code);
                                match self.insert_apl(_hct.clone()) {
                                    Some(ins_res) => Some(_hct),
                                    None => None,
                                }
                            }
                            None => None,
                        }
                    }
                }
            }
            None => None,
        }
    }

    pub fn decrypt(&self, _ct: hopeCT, _dk: PaillierDecryptionKey, _ek: PaillierEncryptionKey) -> BigInt {
        Paillier::decrypt(&_dk, &_ek, &_ct._c)
    }

    pub fn add(&mut self, _ct1: &hopeCT, _ct2: &hopeCT) -> Option<hopeCT> {
        if let Some(ek) = self.enc_key() {
            let _g1 = _ct1._g + _ct2._g;
            match self.lookup_apl(_g1) {
                Some(_ct) => return Some(_ct),
                None => {
                    let _h1 = pairing(_g1, self._sp._q);
                    let _id = ObjectId::new().unwrap();
                    let _c = Paillier::rerandomize(&ek, &Paillier::add(&ek, &_ct1._c, &_ct2._c));
                    let leaf = hopeLeaf::new(_id.clone(), _c, 0);
                    self.insert_tree(leaf);
                    self.update_tree();
                    match self.lookup_tree(_id.clone()) {
                        Some(_code) => {
                            let _hct = hopeCT::from_id(_id, _c, _g1, _h1, _code);
                            match self.insert_apl(_hct.clone()) {
                                Some(ins_res) => return Some(_hct),
                                None => return None,
                            }
                        }
                        None => return None,
                    }
                }
            }
        }
        None
    }
 
    pub fn sub(&mut self, _ct1: &hopeCT, _ct2: &hopeCT) -> Option<hopeCT> {
        if let Some(ek) = self.enc_key() {
            let _g1 = _ct1._g - _ct2._g;
            match self.lookup_apl(_g1) {
                Some(_ct) => return Some(_ct),
                None => {
                    let _id = ObjectId::new().unwrap();
                    match Paillier::sub(&ek, &_ct1._c, &_ct2._c) {
                    	Some(result) => {
		                    let _c = Paillier::rerandomize(&ek, &result);
		                    let leaf = hopeLeaf::new(_id.clone(), _c.clone(), 0);
		                    self.insert_tree(leaf);
		                    match self.lookup_tree(_id.clone()) {
		                        Some(_code) => {
		                            let _h1 = pairing(_g1, self._sp._q);
		                            let _hct = hopeCT::from_id(_id, _c, _g1, _h1, _code);
		                            match self.insert_apl(_hct.clone()) {
		                                Some(ins_res) => return Some(_hct),
		                                None => return None,
		                            }
		                        }
		                        None => return None,
		                    }
                    	},
                    	None => return None,
                    }
                }
            }
        }
        None
    }


    //pub fn ask_client<T>(_req: &ProtocolReq<T>, ctx: &mut Self::Context) -> ProtocolRes<T> {
    //Paillier::decrypt(_pk._key, _ct._c);
    //}


    pub fn fetch_ct(&self, _id: ObjectId) -> Option<hopeCT> {
        None
    }

    pub fn insert_ct(&self, _ct: hopeCT) -> Option<ObjectId> {
        None
    }

    pub fn insert_tree(&mut self, _elem: hopeLeaf) {
        self._tree.insert(_elem);
    }

    pub fn update_tree(&self) {
        //self._tree.update_apl(&MONGO.collection(&self._coll))
    }

    pub fn lookup_tree(&self, _id: ObjectId) -> Option<u64> {
        self._tree.code(_id)
    }

    pub fn lookup_apl(&self, _token: bn::G1) -> Option<hopeCT> {
        match serde_json::to_string(&_token) {
            Err(_) => return None,
            Ok(_g) => {
                match self._apl.get(&_g) {
                    Some(_id) => return self.fetch_ct(_id.clone()),
                    None => return None,
                }
            }
        }
    }

    pub fn insert_apl(&mut self, _elem: hopeCT) -> Option<ObjectId> {
        match serde_json::to_string(&_elem._g) {
            Err(_) => return None,
            Ok(_g) => self._apl.insert(_g, _elem._id),
        }
    }

    //pub fn lookup_ppl(&self, _token: Document) -> Option<hopeCT> {}
    // omitted

    //pub fn insert_ppl(&self, _elem: hopeCT) -> Option<InsertOneResult> {}
    // omitted

    pub fn keys(&self) -> Option<hopeK> {
        return self._key.clone();
    }

    pub fn enc_key(&self) -> Option<PaillierEncryptionKey> {
        if let Some(ref _k) = &self._key {
            return Some(_k._ek.clone());
        }
        None
    }

    pub fn dec_key(&self) -> Option<PaillierDecryptionKey> {
        if let Some(ref _k) = &self._key {
            return _k._dk.clone();
        }
        None
    }
}
/*


impl AddAssign for hopeCT<T> {
    fn add_assign(&mut self, other: Self) {
        let _g1 = self._g.add(other._g);
        match lookup_apl(_g1) {
            Some(_ct) => {
                return _ct;
            }
            None => {
                let _c = Paillier::add(&_pk._key, self._c, other._c);
                let _ct = hopeCT<T> {
                    id: bson::oid::ObjectId::new(),
                    _c: _c,
                    _g: _g1,
                    _h: pairing(_g1, super::_p),
                    _o: lookup_code(_c),
                };
                match insert_apl(_ct) {
                    Some(res) => {
                        match insert_code(_ct) {
                            Some(res) => *self = _ct,
                            None => {
                                panic!("this should not happen!");
                            }
                        }
                    }
                    None => {
                        panic!("this should not happen!");
                    }
                }
            }
        }
    }
}

impl StdSub for hopeCT<T> {
    type Output = hopeCT<T>;

    fn sub(self, other: Point) -> hopeCT<T> {
        Point {
            //x: self.x - other.x,
            //y: self.y - other.y,
            id: bson::oid::ObjectId::new(),
            _c: _ct,
            _g: _g,
            _h: _h,
            _o: _o,
        }
    }
}

impl SubAssign for hopeCT<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            //x: self.x + other.x,
            //y: self.y + other.y,
            id: bson::oid::ObjectId::new(),
            _c: _ct,
            _g: _g,
            _h: _h,
            _o: _o,
        };
    }
}

impl std::ops::Add for hopeCT<T> {
    type Output = hopeCT<T>;

    fn add(self, other: hopeCT<T>) -> hopeCT<T> {
        if let Some(ek) = System::enc_key() {
            let _g1 = self._g.add(other._g);
            match super::lookup_apl(_g1) {
                Some(_ct) => {
                    return _ct;
                }
                None => {
                    let _addition = Paillier::add(&ek._key, self._c, other._c);
                    Paillier::rerandomize(&ek._key, _addition);
                    let _ct = hopeCT<T> {
                        _id: ObjectId::new().unwrap(),
                        _c: _addition,
                        _g: _g1,
                        _h: pairing(_g1, super::_q),
                        _o: super::lookup_code(_c),
                    };
                    match super::insert_apl(_ct) {
                        Some(res) => {
                            match super::insert_code(_ct) {
                                Some(res) => _ct,
                                None => {
                                    panic!(
                                        "PANIC ! this should not happen! coould not insert code"
                                    );
                                }
                            }
                        }
                        None => {
                            panic!("PANIC !this should not happen! could not insert apl");
                        }
                    }
                }
            }
        } else {
            panic!("PANIC ! no encryption key found =(")
        }
    }
}
*/


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn and() {

        //        assert_eq!(_match.unwrap(), _plaintext);
    }
}
