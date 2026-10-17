fn main(){
    let gym_reps = 10;
    println!("gym reps: {}", gym_reps);

    //gym_reps = 15;      // error: cannot assign a new value

    let mut new_gym_reps = 15;
    println!("gym reps: {}", new_gym_reps);
    new_gym_reps +=1;
    println!("gym reps: {}", new_gym_reps);

}