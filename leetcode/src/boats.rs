/*
You are given an array people where people[i] is the weight of the ith person, 
and an infinite number of boats where each boat can carry a maximum weight of limit. 
Each boat carries at most two people at the same time, 
provided the sum of the weight of those people is at most limit.
Return the minimum number of boats to carry every given person.

Input: people = [3,2,2,1], limit = 3
Output: 3
Explanation: 3 boats (1, 2), (2) and (3)

Input: people = [3,5,3,4], limit = 5
Output: 4
Explanation: 4 boats (3), (3), (4), (5)

int numRescueBoats(vector<int>& people, int limit)
*/

use std::vec::Vec;

struct Boat {
	limit: u8,
	passengers : Vec<u8>,
	filled : bool
}

fn create_new_boat(lim:u8) -> Boat {
	Boat {
		limit:lim,
		passengers : Vec::new(),
		filled : false
	}
}

fn add_person( boat: &mut Boat, personwgt: u8) -> bool {
	if boat.filled { 
		return false;
	}
	let sum : u8 = boat.passengers.iter().sum();
	if sum + personwgt > boat.limit {
		return false;
	}
	boat.passengers.push(personwgt);
	true
}

pub fn num_rescue_boats( people: Vec<u8>, limit: u8 ) -> usize {
	let mut boats : Vec<Boat> = Vec::new();
	let mut idx = 0usize;
	let mut bidx = 0usize;
	let mut lastinsert:bool = false;
	boats.push(create_new_boat(limit));
	while idx < people.len() {	
		while bidx < boats.len() {
			lastinsert = add_person(&mut boats[bidx], people[idx]);
			if lastinsert { 
				idx = idx + 1; 
				bidx = 0usize;
				continue;
			}
			else {
				bidx = bidx + 1;
			}
		}
		if !lastinsert {
			boats.push(create_new_boat(limit));
			add_person(&mut boats[bidx], people[idx]); 
			//We added a new element. So bidx should be available
			//assumption: each person wgt is less than the limit Hence the add_person call
			//on a new boat should always succeed.
			idx = idx + 1; 
			bidx = 0usize;
		}
	}
	boats.len()
} 