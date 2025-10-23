fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) 
{
    for ind1 in 0..numbers.len()
    {
        for ind2 in ind1+1..numbers.len()
        {
            if numbers[ind1] + numbers[ind2] == target
            {
                return (ind1, ind2);
            }
        }
    }
    return (0,0);
}


fn main()
{
    println!("{:?}", two_sum(&[2,2,3], 4));
}
