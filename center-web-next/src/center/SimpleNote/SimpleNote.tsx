import NoteHeader from "./NoteHeader";
import NoteList from "./NoteList";
import NoteContent from "./NoteContent";

function SimpleNote() {
    return (
        <div>
            <p>hello</p>
            <NoteHeader />
            <NoteList />
            <NoteContent />
        </div>
    )
}

export default SimpleNote;