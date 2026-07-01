fn popularity_analysis(scores: Vec<i32>) -> bool {
   let mut increasing = true;
   let mut decreasing = true;
   for i in 0..scores.len()-1 {
       if scores[i] > scores[i+1]{
           increasing = false;
       }
       if scores[i] < scores[i+1]{
               decreasing = false;
       }
   }
   return increasing || decreasing;
}