import React from 'react';
import '../styles/App.css';
import { Link } from 'react-router-dom';

class Choose extends React.Component {

    render() {
        return(
            <div>
                <h1>Roll for: </h1>
                <p>Claws in your leg leave fur on owners clothes. Love you, then bite you push your water glass on the floor or eat a rug and furry furry hairs everywhere oh no human coming lie on counter don't get off counter. Claws in your leg leaves on the floor or eat a rug and furry furry hairs everywhere o</p>
                <div className="btnRow" >
                <Link exact to="/"><button className="spaceRight"> Return
                    </button></Link>
                    </div>
            </div>
        )
    }
}

export default Choose;
