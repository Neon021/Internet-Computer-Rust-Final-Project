use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

#[derive(CandidType, Deserialize)]
struct Exam {
    out_of: u8,
    course: String,
    curve: u8,
}

impl Storable for Exam {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
const MAX_VALUE_SIZE: u32 = 100;
impl BoundedStorable for Exam {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}
type Memory = VirtualMemory<DefaultMemoryImpl>;
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static EXAM_MAP: RefCell<StableBTreeMap<u64, Exam, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
    static PARTICIPATION_PERCENTAGE_MAP: RefCell<StableBTreeMap<u64, u64, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}

#[ic_cdk_macros::query]
fn getExam(key: u64) -> Option<Exam> {
    EXAM_MAP.with(|p| p.borrow().get(&key))
}
#[ic_cdk_macros::query]
fn getParticipation(key: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk_macros::update]
fn insertExam(key: u64, value: Exam) -> Option<Exam> {
    EXAM_MAP.with(|p| p.borrow_mut().insert(key, value))
}

#[ic_cdk_macros::update]
fn insertParticipation(key: u64, value: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow_mut().insert(key, value))
}
#[ic_cdk_macros::update]
fn updateExam(key: u64, new_exam_details: Exam) -> Option<Exam> {
    //Check if the value we got from the EXAM_MAP by wrapping it with Some IOption<T> type
    if let Some(existing_exam) = getExam(key) {
        insertExam(key, new_exam_details);//If a value is present then we can insert the parameter value into the existing_exam
        Some(existing_exam)
    } else {
        None // Return None if the exam doesn't exist
    }
}