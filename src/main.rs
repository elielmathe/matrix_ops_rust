fn main() {
    println!("Some Matrix manipulation");

    let a: [i32;5] = [4,43,54,54,54];
    let b: [i32;5] = [3,23,43,53,34];
    let mut c:[i32;5] = [0;5];
    let mut x:usize = 0;




    let  d = array_add(&a,&b);

    match d{
        Some(v) => {
            println!("{:?}",v);
            println!("Good");
        },
        None => println!("The values are incompatible")
    }

    {
        let a = [[23,32,324,34],[43,43,23,23]];
        let b = [[23,2],[324,4],[43,43],[23,23]];
        match array_mult(&a,&b){
            Some(v) => {
                println!("{:?}",v)
            },
            None => println!("Invalid arrays")
        }
    }

    {
        let a = vec![vec![23,32,324,34],vec![43,43,23,23]];
        let b = vec![vec![23,2],vec![324,4],vec![43,43],vec![23,23],vec![32,32]];
        match array_mult_1(&a,&b){
            Some(v) => {
                println!("{:?}",v)
            },
            None => println!("Invalid arrays")
        }
    }
}

fn array_add(a:&[i32],b:&[i32]) -> Option<Vec<i32>>{
    if a.len() == b.len(){
        let x:usize = a.len();
        let mut vc = Vec::new();
        let mut y:usize = 0;
        for i in a.iter(){
            vc.push(a[y] + b[y]);
            y += 1;
        }
        Some(vc)
    }else{
        None
    }
}

fn array_mult(a:&[[i32;4];2],b:&[[i32;2];4]) -> Option<Vec<Vec<i32>>>{
    if  a.len() == b[0].len() && a[0].len() == b.len(){
        let mut c = vec![vec![0;b.len()];a.len()];
        let mut x : usize = 0;
        let mut y : usize = 0;
        let mut z : usize = 0;

        for i in a.iter(){
            z = 0;
            for k in i.iter(){
                y = 0;
                for j in b[x].iter(){
                    c[x][z] = a[x][z] * b[x][y];
                    y+= 1;
                }
                z+= 1;
            }

            x+= 1;
        }
        Some(c)
    }else {
        None
    }
}


fn array_mult_1(a:&Vec<Vec<i32>>,b:&Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>>{
    if  a.len() == b[0].len() && a[0].len() == b.len(){
        let mut c = vec![vec![0;b.len()];a.len()];
        let mut x : usize = 0;
        let mut y : usize = 0;
        let mut z : usize = 0;

        for i in a.iter(){
            z = 0;
            for k in i.iter(){
                y = 0;
                for j in b[x].iter(){
                    c[x][z] = a[x][z] * b[x][y];
                    y+= 1;
                }
                z+= 1;
            }

            x+= 1;
        }
        Some(c)
    }else {
        None
    }
}

fn addition(a : i32,b : i32) -> i32 {
    a + b
}


mod matrix_tests{
    #[test]
    fn basic_test(){
        assert!(1 == 1);
    }

    #[test]
    fn test_addition(){
        let testList = [[[1,1],[2,2],[3,3]],[[2,2],[3,4],[5,6]],[[-2,-100],[98,90],[96,-10]]];
        for i in testList.iter(){
            match super::array_add(&i[0],&i[1]){
                Some(v) => assert_eq!(v,i[2].to_vec()),
                None => panic!()
            }
        }


    }

    #[test]
    #[ignore]
    fn test_multiplication(){
        assert_eq!(1,1);
        //Not yet implemented
    }

    #[test]
    fn test_add(){
        assert_eq!(super::addition(1,1),2);
        assert_eq!(super::addition(2,1),3);
        assert_eq!(super::addition(200000,12343),212343);
    }
}
