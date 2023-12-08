import './Label.css';


function Label() {
    return (
        //<div class="columns">
        //    <p class="column">hello labels</p>
        //    <button class="column button is-primary">clickme</button>
        //</div>
        <div className="navbar-menu">
            <div className="navbar-start">
                <div className="navbar-item">
                    Home
                </div>
            </div>

            <div className="navbar-end">
                <div className="navbar-item">
                    {/*<img src="https://bulma.io/images/bulma-logo.png" width="112" height="28" alt="Bulma"></img>*/}
                </div>
            </div>
        </div>
    );
}

export default Label;
