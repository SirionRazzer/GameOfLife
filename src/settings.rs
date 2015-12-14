use ruleset::Ruleset;

use rustty::{
    Terminal, 
    Event,
    HasSize
};

use rustty::ui::core::{
    Widget,
    HorizontalAlign,
    VerticalAlign,
    ButtonResult
};

use rustty::ui::{
    Dialog,
    Label,
    StdButton
};

pub fn open(_ruleset: &mut Ruleset, term: &mut Terminal) {
    let (t_width, t_height) = term.size();

    let mut ui_ = create_ui(t_width/2, t_height/2, &_ruleset);
    ui_.pack(term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
    
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ui_.result_for_key(ch) {
                Some(ButtonResult::Ok) => break 'main,
                _                      => {}
            }
        }

        ui_.draw(term);
        term.swap_buffers().unwrap();
    }
}

fn create_ui(width: usize, height: usize,rules: &Ruleset) -> Dialog {
    let mut dlg = Dialog::new(width, height);
    dlg.draw_box();

    let mut title = Label::from_str("Settings");
    title.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,1));
    dlg.add_label(title);

    let category1_s = 6;
    let mut category1 = Label::new(width/3, 3);
    category1.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    category1.set_text("Generational Rules: ".to_string() 
                       + &(0..width/3-1).map(|_| "─").collect::<String>());
    category1.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2, 3));
    dlg.add_label(category1);

    let mut starvation_b = StdButton::new(&format!("{:<15} [{}]", "Starvation", rules.starvation), 's', ButtonResult::Custom(1));
    starvation_b.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,category1_s));
    dlg.add_button(starvation_b);

    let mut living_b = StdButton::new(&format!("{:<15} [{}]", "living", rules.living), 'l', ButtonResult::Custom(2));
    living_b.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,category1_s+1));
    dlg.add_button(living_b);

    let mut smothered_b = StdButton::new(&format!("{:<15} [{}]", "Smothered", rules.smothered), 'm', ButtonResult::Custom(3));
    smothered_b.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,category1_s+2));
    dlg.add_button(smothered_b);

    let mut born_b = StdButton::new(&format!("{:<15} [{}]", "Born", rules.born), 'b', ButtonResult::Custom(4));
    born_b.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,category1_s+3));
    dlg.add_button(born_b);

    let category2_s = category1_s+7;
    let mut category2 = Label::new(width/3, 3);
    category2.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    category2.set_text("Randomize ".to_string()
                       + &(0..width/3-1).map(|_| "─").collect::<String>());
    category2.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2, category1_s+5));
    dlg.add_label(category2);
   
    let mut dist_b = StdButton::new(&format!("Percent of Alive Cells Distribution [{}]", rules.distribution), 'p', ButtonResult::Custom(5));
    dist_b.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2, category2_s+1));
    dlg.add_button(dist_b);

    let mut quit = StdButton::new("Quit", 'q', ButtonResult::Ok);
    quit.pack(&dlg, HorizontalAlign::Right, VerticalAlign::Bottom, (2,1));
    dlg.add_button(quit);

    dlg
}
