use std::collections::HashMap;

// O(n ^ 2) time | O(n ^ 2) space
pub fn stable_internships(interns: &[&[usize]], teams: &[&[usize]]) -> Vec<(usize, usize)> {
    let mut free_interns = vec![];
    for i in 0..interns.len() {
        free_interns.push(i);
    }

    let mut current_intern_choices = vec![0; interns.len()];

    let mut choosen_interns: HashMap<usize, usize> = HashMap::new();

    let mut team_maps = vec![];
    for team in teams.iter() {
        let mut rank = HashMap::new();
        for (i, t) in team.iter().enumerate() {
            rank.insert(*t, i);
        }
        team_maps.push(rank);
    }

    while let Some(current_intern) = free_interns.pop() {

        let intern = interns[current_intern];

        let team_preference = intern[current_intern_choices[current_intern]];

        current_intern_choices[current_intern] += 1;

        if let Some(&prev_intern) = choosen_interns.get(&team_preference) {
            if let Some(rank) = team_maps.get(team_preference) {
                if let (Some(current_intern_rank), Some(prev_intern_rank)) = (rank.get(&current_intern), rank.get(&prev_intern)) {
                    if current_intern_rank < prev_intern_rank {
                        choosen_interns.insert(team_preference, current_intern);
                        free_interns.push(prev_intern);
                    } else {
                        free_interns.push(current_intern);
                    }
                }
            }         
        } else {
            choosen_interns.insert(team_preference, current_intern);
        }        
    }

    let mut res = vec![];

    for (k, v) in choosen_interns {
        res.push((v, k));
    }
    
    res
}

#[cfg(test)]
mod tests {

    use super::stable_internships;

    #[test]
    fn it_stable_internships() {

        let interns: &[&[usize]] = &[
            &[0, 1, 2],
            &[0, 2, 1],
            &[1, 2, 0],
        ];

        let teams: &[&[usize]] = &[
            &[2, 1, 0],
            &[0, 1, 2],
            &[0, 1, 2]
        ];

        let mut res = stable_internships(interns, teams);
        res.sort_by_key(|x| x.0);

        println!("{:?}", res);

        assert_eq!(res, vec![(0, 1), (1, 0), (2, 2)])
    }
}