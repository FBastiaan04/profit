use rand::distributions::{Distribution, Uniform};

#[derive(Clone,Copy,Debug)]
struct Job {
	start_time: u8,
	end_time: u8,
	profit: u32,
}

impl Job {
	fn new(start_time: u8, end_time: u8, profit: u32) -> Self {
		return Self {start_time, end_time, profit}
	}
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let profit = self.profit.to_string();
        write!(f, "{}|{}{}|", " ".repeat(self.start_time as usize * 2), profit, " ".repeat((self.end_time - self.start_time) as usize * 2 - profit.len()))
    }
}

#[derive(Debug)]
struct Node {
	job: Job,
	total_profit: u32,
    best_child_index: Option<usize>,
	children: Vec<Node>,
}

impl Node {
	fn new(job: Job, children: Vec<Node>, total_profit: u32, best_child_index: Option<usize>) -> Self {
		Self {job, children, total_profit, best_child_index }
	}
}

fn gen_random_job(rng: &mut rand::rngs::ThreadRng, start_time_dist: Uniform<u8>, profit_dist: Uniform<u32>) -> Job {
    let start_time = start_time_dist.sample(rng);
    let end_time_dist = Uniform::from((start_time + 1)..10);
    let end_time = end_time_dist.sample(rng);
    let profit = profit_dist.sample(rng);
    return Job::new(start_time, end_time, profit)
}

fn get_options(previous_end_time: u8, jobs: &Vec<Job>) -> Vec<Node> {
    // first, remove all jobs starting before the previous end time
    let jobs_left: Vec<Job> = jobs
        .iter()
        .filter(|&job| job.start_time > previous_end_time)
        .copied()
        .collect();
    
    // second, return nothing if there are no jobs after the last end time
    if jobs_left.len() == 0 {
        return vec![];
    }

	// third, find earliest end time. The next job must start before this time
	let earliest_end_time = jobs_left.iter().min_by(|a, b| a.end_time.cmp(&b.end_time)).unwrap().end_time;
	

	jobs_left
		.iter()
		.filter(|&j| j.start_time <= earliest_end_time)
		.map(|&job| {
			let children = get_options(job.end_time, &jobs_left);
            let (total_profit, best_child_index) = match children
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.job.profit.cmp(&b.job.profit)) {
                    Some((best_child_index, best_child)) => (best_child.total_profit + job.profit, Some(best_child_index)),
                    None => (job.profit, None)
                };
			Node::new(job, children, total_profit, best_child_index)
		})
		.collect()
}

fn main() {
    let start_time_dist = Uniform::from(0..9);
    let profit_dist = Uniform::from(1..100);
    let mut rng = rand::thread_rng();
    let jobs = (0..10)
        .into_iter()
        .map(|_| gen_random_job(&mut rng, start_time_dist, profit_dist))
        .collect();
    for &job in &jobs {
        println!("{}", job);
    }
    for i in 0..10 {
        print!("{}|", i);
    }
    println!();
    let branches = get_options(0, &jobs);
    //println!("{:#?}", branches);
    let mut current_best_branch = branches.iter().max_by(|a, b| a.job.profit.cmp(&b.job.profit)).unwrap();

    println!("Done! Total profit: {}", current_best_branch.total_profit);
    println!("{}", current_best_branch.job);
    while let Some(best_child_index) = current_best_branch.best_child_index {
        current_best_branch = &current_best_branch.children[best_child_index];
        println!("{}", current_best_branch.job);
    }
    println!();
}

