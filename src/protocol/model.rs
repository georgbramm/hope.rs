use serde::{Deserialize, Serialize};
use bn::*;
use mongodb::{oid::ObjectId, coll::Collection};
use bson::*;
use std::ops::Shl;
use std::cmp::Ordering;
use std::marker::PhantomData;
use num_bigint::*;


/// ehOPE Node of T
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct hopeNode {
    /// node id
    pub _id: ObjectId,
    /// degree of the node
    pub _degree: usize,
    /// current filling of the node
    pub _num_cts: usize,
    /// type of this node: leaf or inner
    pub _is_leaf: bool,
    /// vector of child nodes
    pub _children: Vec<hopeNode>,
    /// vector of child leafs
    pub _cts: Vec<hopeLeaf>,
}

/// ehOPE Leaf of T
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct hopeLeaf {
    /// node id
    pub _id: ObjectId,
    /// paillier ciphertext
    pub _c: BigInt,
    /// B^+ code
    pub _o: u64,
}

/// ehOPE System Parameters
#[derive(Serialize, Deserialize, Clone)]
pub struct hopeSP {
    /// id of the SP
    pub _id: ObjectId,
    /// name of the ehOPE scheme
    pub _name: String,
    /// G1 generator of the ehOPE scheme
    pub _p: G1,
    /// G2 generator of the ehOPE scheme
    pub _q: G2,
}

/// ehOPE Ciphertext (CT)
#[derive(Serialize, Deserialize, Clone)]
pub struct hopeCT {
    /// id of the CT
    pub _id: ObjectId,
    /// paillier ciphertext
    pub _c: BigInt,
    /// G element
    pub _g: G1,
    /// H element
    pub _h: Gt,
    /// B^+ code
    pub _o: u64,
}

/// A ehOPE PAILLIER KEY PAIR (EK/DK)
#[derive(Serialize, Deserialize, Clone)]
pub struct PaillierDecryptionKey {
    pub lambda: BigInt,
    pub mu: BigInt,
}

/// A ehOPE PAILLIER KEY PAIR (EK/DK)
#[derive(Serialize, Deserialize, Clone)]
pub struct PaillierEncryptionKey {
    pub n: BigInt,
    pub n2: BigInt,
    pub g: BigInt,
}

/// A ehOPE PAILLIER KEY PAIR (EK/DK)
#[derive(Serialize, Deserialize, Clone)]
pub struct hopeK {
    /// the decryption key
    pub _dk: Option<PaillierDecryptionKey>,
    /// the encryption key
    pub _ek: PaillierEncryptionKey,
}

impl hopeSP {
    pub fn new(_name: String) -> hopeSP {
        // return SP
        let g1: G1 = G1::random();
        let g2: G2 = G2::random();
        hopeSP {
            _id: ObjectId::new().unwrap(),
            _name: _name,
            _p: g1,
            _q: g2,
        }
    }
}

impl hopeK {
    pub fn new(ek: PaillierEncryptionKey, dk: PaillierDecryptionKey) -> hopeK {
        // return hopeKey
        hopeK {
            _dk: Some(dk),
            _ek: ek,
        }
    }
}

impl hopeCT {
    pub fn clone(_id: ObjectId, _c: BigInt, _g: G1, _h: Gt, _o: u64) -> hopeCT {
        hopeCT {
            _id: _id,
            _c: _c,
            _g: _g,
            _h: _h,
            _o: _o,
        }
    }

    pub fn from_id(_id: ObjectId, _c: BigInt, _g: G1, _h: Gt, _o: u64) -> hopeCT {
        hopeCT {
            _id: _id,
            _c: _c,
            _g: _g,
            _h: _h,
            _o: _o,
        }
    }

    pub fn new(_c: BigInt, _g: G1, _h: Gt, _o: u64) -> hopeCT {
        hopeCT {
            _id: ObjectId::new().unwrap(),
            _c: _c,
            _g: _g,
            _h: _h,
            _o: _o,
        }
    }
}

impl Ord for hopeCT {
    fn cmp(&self, other: &Self) -> Ordering {
        self._o.cmp(&other._o)
    }
}

impl PartialOrd for hopeCT {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for hopeCT {
    fn eq(&self, other: &Self) -> bool {
        self._o == other._o
    }
}

impl Eq for hopeCT {}

impl hopeNode {
    pub fn new(_degree: usize, _is_leaf: bool) -> hopeNode {
        if _is_leaf {
            hopeNode {
                _id: ObjectId::new().unwrap(),
                _degree: _degree,
                _num_cts: 0,
                _is_leaf: true,
                _children: Vec::with_capacity(_degree - 1),
                _cts: Vec::with_capacity(_degree),
            }
        } else {
            hopeNode {
                _id: ObjectId::new().unwrap(),
                _degree: _degree - 1,
                _num_cts: 0,
                _is_leaf: false,
                _children: Vec::with_capacity(_degree - 1),
                _cts: Vec::with_capacity(_degree),
            }
        }
    }

    pub fn capacity(&self) -> usize {
        self._degree
    }

    pub fn insert_key(&mut self, _key: hopeLeaf) -> usize {
        let mut i = 0;
        while i < self._num_cts {
            //&& super::super::ask_client(key > self._cts[i]) {
            i += 1;
        }
        self._num_cts += 1;
        self._cts.insert(i, _key);
        i
    }

    pub fn remove_key(&mut self, _key: hopeLeaf) -> hopeLeaf {
        let mut i = 0;
        while i < self._num_cts && self._cts[i] != _key {
            i += 1;
        }
        self._num_cts -= 1;
        return self._cts.remove(i);
    }

    pub fn is_leaf(&self) -> bool {
        self._is_leaf
    }

    pub fn is_full(&self) -> bool {
        if self.is_leaf() {
            self._cts.len() == self._degree
        } else {
            self._children.len() == self._degree
        }
    }

    pub fn split(self) -> (hopeLeaf, hopeNode, hopeNode) {
        let _key: hopeLeaf = self._cts[self._degree - 1].clone();
        let mut left = hopeNode::new(self._degree, self._is_leaf);
        let mut right = hopeNode::new(self._degree, self._is_leaf);

        for (index, k) in self._cts.iter().enumerate() {
            if index < self._degree - 1 {
                left.insert_key(k.clone());
            } else if index > self._degree - 1 {
                right.insert_key(k.clone());
            }
        }
        let mut index = 0;
        for child in self._children {
            if index < self._degree {
                left._children.push(child);
            } else {
                right._children.push(child);
            }
            index += 1;
        }
        return (_key, left, right);
    }

    pub fn search(&self, _key: hopeLeaf) -> Option<&hopeNode> {
        // Find the first key greater than or equal to k
        let mut i = 0;
        while i < self._num_cts {
            //&& super::super::ask_client(key > self._cts[i]) {
            // If key is on this node, return the node
            if self._cts[i] == _key {
                return Some(self);
            }
            i += 1;
        }
        if self._is_leaf == true {
            return None;
        }
        return self._children[i].search(_key);
    }

    pub fn code(&self, _code: u64, _key: ObjectId) -> Option<u64> {
        // Find the first key greater than or equal to k
        let mut i = 0;
        while i < self._num_cts {
            //&& super::super::ask_client(key > self._cts[i]) {

            // If key is on this node, return the node
            if self._cts[i]._id == _key {
                let _code = (_code + u64::from(i as u32)).shl(self.capacity());
                Some(_code);
            }
            i += 1;
        }
        if self._is_leaf == true {
            return None;
        }
        return self._children[i].code((_code + u64::from(i as u32)).shl(self.capacity()), _key);
    }

    pub fn update_apl(&self, _code: u64, _coll: &Collection) {
        let mut i = 0;
        if self._num_cts == 0 {
            while i < self._children.len() {
                let _current_code = (_code + u64::from(i as u32)).shl(self.capacity());
                self._children[i].update_apl(_current_code, _coll);
                i += 1;
            }
        } else {
            i = 0;
            while i < self._num_cts {
                _coll
	                .update_one(
	                    doc!{"_id" => self._cts[i]._id.clone()},
	                    
	                    doc!{"$set" => {"_o" => (_code + u64::from(i as u32)).shl(self.capacity())}},
	                        None,
                    )
                    .unwrap();
	            i += 1;
            }
        }
    }
}

impl hopeLeaf {
    pub fn new(_id: ObjectId, _c: BigInt, _o: u64) -> hopeLeaf {
        hopeLeaf {
            _id: _id,
            _c: _c,
            _o: _o,
        }
    }
}

impl Ord for hopeLeaf {
    fn cmp(&self, other: &Self) -> Ordering {
        self._o.cmp(&other._o)
    }
}

impl PartialOrd for hopeLeaf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for hopeLeaf {
    fn eq(&self, other: &Self) -> bool {
        self._o == other._o
    }
}

impl Eq for hopeLeaf {}


impl std::fmt::Display for hopeLeaf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "@leaf[id:{:?}, _o:{:?}, _c:{:?}]",
            self._id,
            self._o,
            self._c,
        )
    }
}

impl std::fmt::Display for hopeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "@node[_degree:{:?}, _num_cts:{:?}, _is_leaf:{:?}, _children:{:?}, _cts:{:?}]",
            self._degree,
            self._num_cts,
            self._is_leaf,
            self._children,
            self._cts
        )
    }
}
