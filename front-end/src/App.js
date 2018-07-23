import React from 'react';
import { Link } from 'react-router-dom';
import './styles/App.css';

class App extends React.Component {
  render() {
      return (
        <div style={{marginTop: '10%'}}>
          <h1>Upgraded Pancakes</h1>
          <p>Claws in your leg leave fur on owners clothes. Love you, then bite you push your water glass on the floor or eat a rug and furry furry hairs everywhere oh no human coming lie on counter don't get off counter. Claws in your leg leaves on the floor or eat a rug and furry furry hairs everywhere oh </p>
          <div className="btnRow">
            <div className="spaceRight">
              <Link to="/choose">
                <button> Roll </button>
              </Link>
            </div>
            <Link to="/create">
              <button>Generate Table</button>
            </Link>
          </div>
        </div>
      )
  }
}

export default App;
