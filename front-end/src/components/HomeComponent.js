import React from 'react';
import { Link } from 'react-router-dom';
import '../styles/App.css';

class Home extends React.Component {
    render() {
        return (
            <div>
                <h1>Upgraded Pancakes</h1>
                <p>Claws in your leg leave fur on owners clothes. Love you, then bite you push your water glass on the floor or eat a rug and furry furry hairs everywhere oh no human coming lie on counter don't get off counter. Claws in your leg leaves on the floor or eat a rug and furry furry hairs everywhere oh </p>

                <div className="btnRow" >
                    <Link to="/choose"><button onClick={this.props.changePage}className="spaceRight"> Roll
                    </button></Link>
                    <button className="ripple">Generate Table</button>
                </div>
            </div>
        )
    }
}

export default Home;
