use ncurses::*;
fn main() {
    initscr();
    noecho();
    addstr("Hello World!");
    refresh();
    loop {
        getch();
    }
    endwin();
}
