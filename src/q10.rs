pub mod item_b {
    use std::io::Write;
    use std::collections::{ HashMap, HashSet };
    use std::iter::Iterator;
    use std::cmp::PartialEq;

    #[derive(Hash)]
    struct ContextAux { x: i32, y: i32 }

    #[derive(Clone, Eq)]
    struct Context {
        R1 : i32,
        R2 : i32,
        R3 : i32,
        x  : i32,
        y  : i32
    }

    impl PartialEq for Context {
        fn eq(&self, other: &Self) -> bool {
            if self.x != other.x || self.y != other.y { return false; }
            return true;
        }
    }

    use std::hash::{ Hash, Hasher };
    impl std::hash::Hash for Context {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            let aux = ContextAux { x: self.x, y: self.y };
            aux.hash(hasher)
        }
    }

    struct Command { description: &'static str, action: Box<dyn Fn(&mut Context)> }

    fn generate_permutation(
        solutions: &mut Vec<(Context, Vec<&'static str>)>,
        input: Vec<&'static str>,
        context: Context,
        p1: &[Command],
        p2: &[Command],
        i: usize,
        j: usize
    ) {
        if i == p1.len() && j == p2.len() { // BOTH P1 AND P2 ARE OVER
            solutions.push((context, input));
        }

        else if i == p1.len() { // P1 IS OVER
            let mut context2 = context.clone();
            let mut input2 = input.clone();
            (p2[j].action)(&mut context2);
            input2.push(p2[j].description);
            generate_permutation(solutions, input2, context2, p1, p2, i, j+1);
        }

        else if j == p2.len() { // P2 IS OVER
            let mut context1 = context.clone();
            let mut input1 = input.clone();
            (p1[i].action)(&mut context1);
            input1.push(p1[i].description);
            generate_permutation(solutions, input1, context1, p1, p2, i+1, j);
        }

        else { // BOTH THE PROCESSES HAVE PENDING ACTIONS
            let mut context1 = context.clone();
            let mut input1 = input.clone();
            (p1[i].action)(&mut context1);
            input1.push(p1[i].description);
            generate_permutation(solutions, input1, context1, p1, p2, i+1, j);

            let mut context2 = context.clone();
            let mut input2 = input.clone();
            (p2[j].action)(&mut context2);
            input2.push(p2[j].description);
            generate_permutation(solutions, input2, context2, p1, p2, i, j+1);
        }
    }

    pub fn solution() {
        let initial_context = Context { R1:0, R2:0, R3:0, x:0, y:0 };
        
        let p1 = [
            Command { description: "R1=x", action: Box::new(|ctx: &mut Context| { ctx.R1 = ctx.x; }) },
            Command { description: "R1+=1", action: Box::new(|ctx: &mut Context| { ctx.R1 += 1; }) },
            Command { description: "x=R1", action: Box::new(|ctx: &mut Context| { ctx.x = ctx.R1; }) },
            Command { description: "R1=x", action: Box::new(|ctx: &mut Context| { ctx.R1 = ctx.x; }) },
            Command { description: "R1+=2", action: Box::new(|ctx: &mut Context| { ctx.R1 += 2; }) },
            Command { description: "x=R1", action: Box::new(|ctx: &mut Context| { ctx.x = ctx.R1; }) },
        ];

        let p2 = [
            Command { description: "R2=x", action: Box::new(|ctx: &mut Context| { ctx.R2 = ctx.x; }) },
            Command { description: "R2+=2", action: Box::new(|ctx: &mut Context| { ctx.R2 += 2; }) },
            Command { description: "x=R2", action: Box::new(|ctx: &mut Context| { ctx.x = ctx.R2; }) },
            Command { description: "R2=y", action: Box::new(|ctx: &mut Context| { ctx.R2 = ctx.y; }) },
            Command { description: "R3=x", action: Box::new(|ctx: &mut Context| { ctx.R3 = ctx.x; }) },
            Command { description: "R2-=R3", action: Box::new(|ctx: &mut Context| { ctx.R2 -= ctx.R3; }) },
            Command { description: "y=R2", action: Box::new(|ctx: &mut Context| { ctx.y = ctx.R2; }) },
        ];

        let mut solutions = Vec::new();
        generate_permutation(&mut solutions, vec![], initial_context, &p1, &p2, 0, 0);

        let mut solutions_map = HashMap::new();
        let mut results_map = HashSet::new();

        for (result, description) in solutions {
            let description : String =  description.iter()
                                                   .fold(String::new(), |mut description, action| {
                                                       description.push_str(&format!("{}; ", action));
                                                       description
                                                   });
            solutions_map.insert(description, result.clone());
            results_map.insert(result);
        }

        let mut response = std::fs::File::create("q10b solutions.txt").unwrap();
        for (idx, (k, v)) in solutions_map.iter().enumerate() {
            response.write(format!(
                "Solução {}:\n{}\nx: {}; y: {}\n\n",
                idx, k, v.x, v.y
            ).as_ref()).unwrap();
        }

        for result in results_map {
            println!("x: {}; y: {}; R1: {}; R2: {}, R3: {}", result.x, result.y, result.R1, result.R2, result.R3);
        }
    }
}
