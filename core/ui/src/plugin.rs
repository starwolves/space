use bevy::prelude::{
    App, FixedUpdate, IntoSystemConfigs, IntoSystemSetConfigs, Plugin, PostUpdate, Startup, Update,
};
use networking::messaging::{register_reliable_message, MessageSender};

use crate::{
    button::button_hover_visuals,
    cursor::{
        clear_window_focus_buffer, focus_state, grab_cursor, release_cursor, window_focus_buffer,
        CursorSet, FocusState, GrabCursor, ReleaseCursor, WindowFocusBuffer,
    },
    fonts::{init_fonts, Fonts},
    hlist::{
        clear_freeze_buffer, freeze_button, hlist_created, hlist_input, FreezeBuffer, FreezeButton,
    },
    net::{UiClientMessage, UiServerMessage},
    scrolling::{mouse_scroll, mouse_scroll_inverted},
    text_input::{
        clear_old_focus, focus_events, incoming_messages, input_characters,
        input_mouse_press_unfocus, register_input, set_text_input_node_text, ui_events,
        FocusTextInput, FocusTextSet, SetText, TextInputLabel, TextInputSet, TextTree,
        TextTreeInputSelection, UnfocusTextInput,
    },
};
use resources::{is_server::is_server, sets::MainSet, ui::TextInput};
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                incoming_messages
                    .in_set(TextTree::Input)
                    .in_set(MainSet::Update),
            )
            .add_event::<TextTreeInputSelection>();
        } else {
            app.configure_sets(Update, (FocusTextSet::Focus, FocusTextSet::Unfocus).chain());
            app.init_resource::<WindowFocusBuffer>()
                .init_resource::<FreezeBuffer>()
                .add_systems(PostUpdate, clear_old_focus)
                .add_systems(
                    Update,
                    (
                        input_characters,
                        focus_events
                            .in_set(TextInputLabel::MousePressUnfocus)
                            .after(FocusTextSet::Focus),
                        input_mouse_press_unfocus
                            .before(TextInputLabel::MousePressUnfocus)
                            .before(CursorSet::Perform)
                            .in_set(FocusTextSet::Unfocus),
                        ui_events
                            .before(TextInputLabel::MousePressUnfocus)
                            .in_set(FocusTextSet::Focus),
                    ),
                )
                .add_systems(FixedUpdate, clear_freeze_buffer.in_set(MainSet::PreUpdate))
                .add_systems(
                    FixedUpdate,
                    (
                        button_hover_visuals,
                        set_text_input_node_text.in_set(TextInputSet::Set),
                        mouse_scroll_inverted,
                        mouse_scroll,
                        hlist_input.before(freeze_button),
                        hlist_created.before(freeze_button),
                        freeze_button,
                        focus_state,
                    )
                        .in_set(MainSet::Update),
                )
                .init_resource::<TextInput>()
                .add_event::<UnfocusTextInput>()
                .add_event::<FocusTextInput>()
                .add_event::<SetText>()
                .add_event::<FreezeButton>()
                .add_systems(Startup, register_input)
                .add_systems(
                    Update,
                    (
                        window_focus_buffer.after(TextInputLabel::MousePressUnfocus),
                        release_cursor.in_set(CursorSet::Perform).after(grab_cursor),
                        grab_cursor.in_set(CursorSet::Perform).after(focus_state),
                    ),
                )
                .add_systems(
                    FixedUpdate,
                    clear_window_focus_buffer.in_set(MainSet::PostUpdate),
                )
                .add_event::<GrabCursor>()
                .add_event::<ReleaseCursor>()
                .init_resource::<FocusState>();
        }
        app.init_resource::<Fonts>()
            .add_systems(Startup, init_fonts);
        register_reliable_message::<UiClientMessage>(app, MessageSender::Client);
        register_reliable_message::<UiServerMessage>(app, MessageSender::Server);
    }
}
