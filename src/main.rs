use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    //for middle
    let  vecm = vec![1,5,4,3,3,4,6,5,6,4,2,3,2,4,2,2,3,4,3,3,5,2,2,3,2,2,5,3,2,2,1,3,2,6,3,6,1,4,5,6,4,1,5,6,4,5,5,3,6,2,6,4,4,3,2,3,4,2,3,6,5,4,3,5,5,4,4,5,6,5,2,4,1,3,6,6,5,4,4,6,2,5,3,2,5,2,5,3,1,2,2,6,5,2,5,2,5,4,3,3,2,4,3,5,2,2,3,4,3,1,2,1,3,5,5];
    


    //for root
    let  vec = vec![1,5,4,3,3,4,6,5,6,4,2,3,2,4,2,2,3,4,3,3,5,2,2,3,2,2,5,3,2,2,1,3,2,6,3,6,1,4,5,6,4,1,5,6,4,5,5,3,6,2,6,4,4,3,2,3,4,2,3,6,5,4,3,5,5,4,4,5,6,5,2,4,1,3,6,6,5,4,4,6,2,5,3,2,5,2,5,3,1,2,2,6,5,2,5,2,5,4,3,3,2,4,3,5,2,2,3,4,3,1,2,1,3,5,5,6,3,6,6,2,1,5,3,5,5,5,6,6,3,4,1,5,2,4,5,2,5,2,4,5,3,2,1,4,4,1,5,2,6,3,2,6,3,5,4,2,2,2,2,4,2,1,5,3,1,6,5,6,5,5,3,5,2,1,6,2,1,5,1,4,4,5,2,3,4,3,5,6,5,1,5,5,4,4,1,6,1,1,4,2,5,4,2,4,5,3,2,1,1,2,1,6,6,2,5,1,5,1,1,4,1,5,1,3,5,6,5,3,1,5,6,5,6,2,4,5,2,6,6,3,6,1,4,1,4,1,2,3,5,5,1,6,3,3,6,1,5,1,4,2,6,3,5,4,1,5,3,4,3,2,5,3,1,6,2,5,5,2,5,2,2,3,5,6,2,5,4,3,1,1,2,4,5,3,1,2,2,4,6,5,6,5,1,5,4,4,1,4,4,3,5,4,3,5,4,4,1,5,5,1,2,5,4,4,6,5,4,6,1,2,2,4,3,4,6,6,6,3,4,5,3,1,4,4,2,5,4,6,2,4,5,5,1,2,6,5,2,2,4,4,3,4,4,4,6,4,4,4,3,2,2,3,5,6,1,4,1,4,5,1,3,5,5,5,2,4,3,3,1,2,3,1,4,3,4,2,4,4,5,2,6,6,6,4,2,2,6,1,1,5,3,1,6,6,5,4,6,1,6,1,6,5,3,5,2,4,5,4,6,5,4,5,1,1,6,3,3,2,2,4,6,6,1,1,6,1,2,2,1,5,2,5,2,2,4,6,1,2,1,5,5,6,3,5,1,4,4,3,2,1,2,3,1,4,6,3,3,3,3,4,5,3,3,2,5,5,6,4,6,2,6,5,1,4,2,3,6,5,1,2,2,5,3,2,2,1,3,2,6,3,6,1,4,5,6,4,1,5,6,4,5,5,3,6,2,6,4,4,3,2,2,2,5,3,2,2,1,3,2,6,3,6,1,4,5,6,4,1,5,6,4,5,5,3,6,2,6,4,4,3,2,2,2,5,3,2,2,1,3,2,6,3,6,1,4,5,6,4,1,5,6,4,5,5,3,6,2,6,4,4,3,2,2,2,5,3,2,2,1,3,2,6,3,6,1,4,5,6,4,1,5,6,4,5,5,3,6,2,6,4,4,3,2,2,2,5,3,2,2,1,3,2,6,3,6,1,4,5,6,4,1,5,6,4,5,5,3,6,2,6,4,4,3,2,2,2,5,3,2,2,1,3,2,6,3,6,1,4,5,6,4,1,5,6,4,5,5,3,6,2,6,4,4,3,2,2,2,5,3,2,2,1,3,2,6,3,6,1,4,5,6,4,1,5,6,4,5,5,3,6,2,6,4,4,3,2,2,2,5,3,2,2,1,3,2,6,3,6,1,4,5,6,4,1,5,6,4,5,5,3,6,2,6,4,4,3,2];
    

    let start = BitMapBackend::new("graph1.png", (640, 480)).into_drawing_area(); // for show and tell
    let middle = BitMapBackend::new("graph2.png", (640, 480)).into_drawing_area(); // for show and tell
    let root = BitMapBackend::new("graph3.png", (640, 480)).into_drawing_area(); //main drawing

    //set the colors
    start.fill(&RGBColor(240, 200, 200))?;
    middle.fill(&RGBColor(240, 200, 200))?;
    root.fill(&RGBColor(240, 200, 200))?;


    
    let start= start.apply_coord_spec(RangedCoord::<RangedCoordf32, RangedCoordf32>::new(
        0f32..1f32,
        0f32..1f32,
        (0..640, 0..480),
    )); 
    let middle = middle.apply_coord_spec(RangedCoord::<RangedCoordf32, RangedCoordf32>::new(
        0f32..1f32,
        0f32..1f32,
        (0..640, 0..480),
    ));
    let root = root.apply_coord_spec(RangedCoord::<RangedCoordf32, RangedCoordf32>::new(
        0f32..1f32,
        0f32..1f32,
        (0..640, 0..480),
    ));

    let dot_and_label = |x: f32, y: f32| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
                format!("({:.2},{:.2})", x, y),
                (10, 0),
                ("sans-serif", 15.0).into_font(),
            );
    };

    //draw a dot with the line only
    let dot_only = |x: f32, y: f32| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())

    };

    start.draw(&dot_only(0.35,0.60))?; //start point
    start.draw(&dot_and_label(0.8, 0.8))?; // 1 and 2
    start.draw(&dot_and_label(0.525, 0.30))?; //3 and 4
    start.draw(&dot_and_label(0.25, 0.80))?; // 5 and 6
    //root.draw(&dot_and_label(0.4, 0.6))?; //test

    middle.draw(&dot_only(0.35,0.60))?; //start point
    middle.draw(&dot_and_label(0.8, 0.8))?; // 1 and 2
    middle.draw(&dot_and_label(0.525, 0.30))?; //3 and 4
    middle.draw(&dot_and_label(0.25, 0.80))?; // 5 and 6


    root.draw(&dot_only(0.35,0.60))?; //start point
    root.draw(&dot_and_label(0.8, 0.8))?; // 1 and 2
    root.draw(&dot_and_label(0.525, 0.30))?; //3 and 4
    root.draw(&dot_and_label(0.25, 0.80))?; // 5 and 6

   /* in python for the algo
    if rand_num == 1 or 2:
        cur_x =  (cur_x + 0.5) /2
        cur_y =  (cur_y + 0.6) /2

        then draw the dot

        root.draw(&dot_and_label(cur_x, cur_y))?;
    */


    //set vars
    let mut cur_x = 0.35;
    let mut cur_y = 0.60;

    //for middle
    for i in vecm {
          print!(" curr_x:{}", cur_x);
          print!(" curr_y:{}", cur_y);
        if i == 1 || i == 2{
            cur_x = (cur_x + 0.8) / 2.0;
            cur_y =  (cur_y + 0.8) / 2.0;
        }
        else if i == 3 || i == 4{
            cur_x = (cur_x + 0.525) / 2.0;
            cur_y =  (cur_y + 0.30) / 2.0;
        }
        else if i == 5 || i == 6{
            cur_x = (cur_x + 0.25) / 2.0;
            cur_y =  (cur_y + 0.80) / 2.0;
        }
        //print!(",");
        middle.draw(&dot_only(cur_x, cur_y))?;
        
    }



    //set vars
    let mut cur_x = 0.35;
    let mut cur_y = 0.60;

    //for root
    for i in vec {
          print!(" curr_x:{}", cur_x);
          print!(" curr_y:{}", cur_y);
        if i == 1 || i == 2{
            cur_x = (cur_x + 0.8) / 2.0;
            cur_y =  (cur_y + 0.8) / 2.0;
        }
        else if i == 3 || i == 4{
            cur_x = (cur_x + 0.525) / 2.0;
            cur_y =  (cur_y + 0.30) / 2.0;
        }
        else if i == 5 || i == 6{
            cur_x = (cur_x + 0.25) / 2.0;
            cur_y =  (cur_y + 0.80) / 2.0;
        }
        //print!(",");
        root.draw(&dot_only(cur_x, cur_y))?;
        
    }

    Ok(())
}
