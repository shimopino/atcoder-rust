use proconio::input;

fn main() {
    input! {
        n: usize,
        xyh: [(u32, u32, u32); n],
    }

    let MAX = 100;
    for posY in 0..MAX {
        for posX in 0..MAX {
            let needH = -1;

            for i in 0..n {
                if xyh[i].2 > 0 {
                    let tmp = xyz[i].2 + (posY - xyz[i].1).abs() + (posX - xyz[i].0).abs();
                    if needH == -1 {
                        needH = tmp;
                    } else {
                        if needH != tmp {
                            needH = -2;
                            break;
                        }
                    }
                }
            }
            if needH == -2 {
                continue;
            }

            for i in 0..n {
                if xyz[i].2 == 0 {
                    let dist = (posY - xyz[i].1).abs() + (posX - xyz[i].0).abs();
                    if needH > dist {
                        needH = -2;
                        break;
                    }
                }
            }
            if needH == -2 {
                continue;
            }

            println!("{} {} {}", posX, posY, needH);
        }
    }
}