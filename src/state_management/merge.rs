// use std::{io, arch::x86_64::_SIDD_NEGATIVE_POLARITY, hint::unreachable_unchecked};

// use crate::{file_management::{hash::Hash, directory::Directory}, interface::io::get_serialized_object};

// pub fn merge(parent1: Hash, parent2: Hash) -> io::Result<()> {
//     let dir1: Directory = get_serialized_object(parent1)?;
//     let dir2: Directory = get_serialized_object(parent2)?;    
// }

// pub fn diff(parent1: &Directory, parent2: &Directory) -> io::Result<()> {
//     let mut new = Directory::new();

//     parent1.get_key_value_pairs()
//         .try_for_each(|(path, blobref)| {
//             let blobref1 = parent1.get_file_ref(&path);
//             let blobref2 = parent2.get_file_ref(&path);

//             match (blobref1, blobref2) {
//                 (Some(b1), None) => todo!(),
//                 (Some(b1), Some(b2)) => todo!(),
//                 (_, _) => unreachable!()
//             }

//             Ok(())
//         });

//     parent2.get_key_value_pairs()
//         .try_for_each(|(path, blobref)| {
//             if !parent1.contains_file_ref(&path) && !new.contains_file_ref(&path) {
//                 new.insert_file_ref(&path, blobref);
//             }
//             Ok(())
//         });
// })