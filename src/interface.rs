use macroquad::prelude::*;
use crate::hyperion;
use hyperion::*;

pub enum GameState {
    Menu,
    Animation,
    End,
}

impl GameState {
    pub async fn menu(&mut self) -> Hyperion {
        use macroquad::ui::{hash, root_ui, widgets::InputText};
        clear_background(BLACK);
        let screen_width = screen_width();
    
        // 使用线程局部存储来保持输入内容
        use std::cell::RefCell;
        thread_local! {
            static L_STR: RefCell<String> = RefCell::new(String::from("0.01"));
            static VX0_STR: RefCell<String> = RefCell::new(String::from("0.0"));
            static VY0_STR: RefCell<String> = RefCell::new(String::from("6.5"));
            static X0_STR: RefCell<String> = RefCell::new(String::from("1.35"));
            static Y0_STR: RefCell<String> = RefCell::new(String::from("0.0"));
            static OMEGA0_STR: RefCell<String> = RefCell::new(String::from("0.5"));
            static THETA0_STR: RefCell<String> = RefCell::new(String::from("0.0"));
        }
    
        let mut start_clicked = false;
        
        root_ui().window(hash!(), Vec2::new(screen_width/2.0-150.0, 100.0), Vec2::new(300.0, 350.0), |ui| {
            ui.label(None, "Init Settings");
            ui.separator();
            ui.label(None, "L:");
            L_STR.with(|s| InputText::new(hash!("L")).ui(ui, &mut *s.borrow_mut()));
            ui.label(None, "vx0:");
            VX0_STR.with(|s| InputText::new(hash!("vx0")).ui(ui, &mut *s.borrow_mut()));
            ui.label(None, "vy0:");
            VY0_STR.with(|s| InputText::new(hash!("vy0")).ui(ui, &mut *s.borrow_mut()));
            ui.label(None, "x0:");
            X0_STR.with(|s| InputText::new(hash!("x0")).ui(ui, &mut *s.borrow_mut()));
            ui.label(None, "y0:");
            Y0_STR.with(|s| InputText::new(hash!("y0")).ui(ui, &mut *s.borrow_mut()));
            ui.label(None, "omega0:");
            OMEGA0_STR.with(|s| InputText::new(hash!("omega0")).ui(ui, &mut *s.borrow_mut()));
            ui.label(None, "theta0:");
            THETA0_STR.with(|s| InputText::new(hash!("theta0")).ui(ui, &mut *s.borrow_mut()));
            ui.separator();
            if ui.button(None, "Start Simulation") {
                start_clicked = true;
            }
        });
        
        next_frame().await;
        if start_clicked {
            *self = GameState::Animation;
            Hyperion::new(
                L_STR.with(|s| s.borrow().parse().unwrap_or(0.01)),
                VX0_STR.with(|s| s.borrow().parse().unwrap_or(0.0)),
                VY0_STR.with(|s| s.borrow().parse().unwrap_or(6.5)),
                X0_STR.with(|s| s.borrow().parse().unwrap_or(1.35)),
                Y0_STR.with(|s| s.borrow().parse().unwrap_or(0.0)),
                OMEGA0_STR.with(|s| s.borrow().parse().unwrap_or(0.5)),
                THETA0_STR.with(|s| s.borrow().parse().unwrap_or(0.0))
            )
        } else {
            Hyperion::new(0.01, 0.0, 6.5, 13.27, 0.0, 0.5, 0.)
        }
    }

    pub async fn animation(&mut self, states: &mut HyperionStates) {
        clear_background(BLACK);
        let screen_width = screen_width();
        let screen_height = screen_height();
        let button_width = screen_width / 8.0;
        let button_height = screen_height / 8.0;
        let button_x = 10.;
        let button_y = 0.;
        
        // 绘制中心棕色圆
        let center_x = screen_width / 2.0;
        let center_y = screen_height / 2.0;
        let center_radius = 20.0;
        draw_circle(center_x, center_y, center_radius, BROWN);
        
        // 使用帧率作为步长
        let dt: f64 = get_frame_time() as f64;
        
        // 迭代并绘制hyperion
        if let Some(last_state) = states.states.last() {
            let new_state = last_state.iterate(dt);
            states.states.push(new_state.clone());
            
            // 绘制两个蓝色小球和连线
            let scale = 50.0; // 缩放因子，使轨迹更明显
            let l_scale: f64 = 30.0; // L的缩放因子
            let x1 = center_x + (new_state.x * scale) as f32 + (new_state.L * new_state.theta.cos() * scale * l_scale / 2.) as f32;
            let y1 = center_y - (new_state.y * scale) as f32 - (new_state.L * new_state.theta.sin() * scale * l_scale / 2.) as f32; // 注意y轴方向
            
            // 计算第二个小球的位置（根据theta角度）
            let x2 = center_x + (new_state.x * scale) as f32 - (new_state.L * new_state.theta.cos() * scale * l_scale / 2.) as f32;
            let y2 = center_y - (new_state.y * scale) as f32 + (new_state.L * new_state.theta.sin() * scale * l_scale / 2.) as f32; // 注意y轴方向
            
            // 绘制连线
            draw_line(x1, y1, x2, y2, 2.0, BLUE);
            
            // 绘制两个蓝色小球（半径越小越好）
            let ball_radius = 5.0;
            draw_circle(x1, y1, ball_radius, BLUE);
            draw_circle(x2, y2, ball_radius, BLUE);
            
            // 绘制运动轨迹（白色像素点）
            draw_circle(x1, y1, 1.0, WHITE);
            draw_circle(x2, y2, 1.0, WHITE);
        }
        
        // 绘制所有历史轨迹点
        for state in &states.states {
            let scale = 50.0;
            let x1 = center_x + (state.x * scale) as f32;
            let y1 = center_y - (state.y * scale) as f32;
            let x2 = x1 + (state.L * state.theta.cos() * scale) as f32;
            let y2 = y1 - (state.L * state.theta.sin() * scale) as f32;
            
            // 绘制轨迹点
            draw_circle(x1, y1, 1.0, WHITE);
            draw_circle(x2, y2, 1.0, WHITE);
        }
        
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            if mouse_position.0 >= button_x && mouse_position.0 <= button_x + button_width &&
               mouse_position.1 >= button_y && mouse_position.1 <= button_y + button_height {
                println!("结束模拟");
                *self = GameState::End;
            }
        }
        draw_rectangle(button_x, button_y, button_width, button_height, RED);
        draw_text("End Iteration", button_x + button_width / 2.0 - measure_text("End Iteration", None, 20, 1.0).width / 2.0, button_y + button_height / 2.0, 20.0, WHITE);
        next_frame().await;
    }
    
    pub async fn end(&mut self, states: &HyperionStates, is_saved: &mut bool) {
        clear_background(BLACK);
        let screen_width = screen_width();
        let screen_height = screen_height();
        let button_width = screen_width / 8.0;
        let button_height = screen_height / 8.0;
        let button_x = (screen_width - button_width) / 2.0;
        let button_y = (screen_height - button_height) / 2.0;
        
        // 输出数据到pickle文件
        use std::fs::File;
        use std::io::Write;
        use serde_pickle::to_vec;
        
        // 创建数据目录
        let data_dir = "data";
        if !std::path::Path::new(data_dir).exists() {
            std::fs::create_dir_all(data_dir).unwrap();
        }
        
        // 提取t、theta、omega数据，并添加is_sampled列
        let mut output_data = Vec::new();
        
        // 检测每个周期中x的最大值
        let mut local_maxima = Vec::new();
        let mut in_cycle = false;
        let mut current_max_x = f64::NEG_INFINITY;
        let mut current_max_index = 0;
        
        for (i, state) in states.states.iter().enumerate() {
            if !in_cycle && state.x > 0.0 {
                // 开始新周期
                in_cycle = true;
                current_max_x = state.x;
                current_max_index = i;
            } else if in_cycle {
                if state.x > current_max_x {
                    // 更新当前周期的最大值
                    current_max_x = state.x;
                    current_max_index = i;
                } else if state.x < 0.0 {
                    // 周期结束，记录最大值位置
                    local_maxima.push(current_max_index);
                    in_cycle = false;
                }
            }
        }
        
        // 标记每个周期中的最大值点
        for (i, state) in states.states.iter().enumerate() {
            let is_sampled = local_maxima.contains(&i);
            output_data.push((state.t, state.theta, state.omega, is_sampled));
        }

        // 保存为pickle文件
        if !*is_saved {
            
            let pickle_data = to_vec(&output_data, serde_pickle::SerOptions::default()).unwrap();
            let mut file = File::create("data/hyperion_data.pickle").unwrap();
            file.write_all(&pickle_data).unwrap();
            println!("数据已输出到 data/hyperion_data.pickle");
            *is_saved = true;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            if mouse_position.0 >= button_x && mouse_position.0 <= button_x + button_width &&
               mouse_position.1 >= button_y && mouse_position.1 <= button_y + button_height {
                println!("重新开始");
                *self = GameState::Menu;
            }
        }
    
        draw_rectangle(button_x, button_y, button_width, button_height, GREEN);
        draw_text("Restart", button_x + button_width / 2.0 - measure_text("Restart", None, 20, 1.0).width / 2.0, button_y + button_height / 2.0, 20.0, WHITE);
        
        // 显示输出完成信息
        draw_text("Output finished", button_x + button_width / 2.0 - measure_text("Output finished", None, 20, 1.0).width / 2.0, button_y - 30.0, 20.0, WHITE);
        
        next_frame().await;
    }
}