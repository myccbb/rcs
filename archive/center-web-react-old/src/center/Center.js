import '../external.css';

function Center() {
    document.title = 'center';
    return (
        // <div className="total">
        //     <section className="side-column">
        //         <button className="button is-primary">hello</button>
        //     </section>
        //     <section className="side-column">
        //         <p className="button">hello</p>
        //     </section>
        // </div>
        <div>
            <nav style={{
                borderBottom: "solid 1px",
                paddingBottom: "1rem",
            }}>
            </nav>
        </div>
    )
}

export default Center;
