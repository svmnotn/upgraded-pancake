import React from 'react';
import '../styles/App.css';
import { Link } from 'react-router-dom';

class Choose extends React.Component {
    render() {
        return(
            <div>
                <h1>Roll for: </h1>
                <hr/>
                <div className="btnRow" style={{marginBottom: 10,}}>
                    <div className="spaceRight">
                        <button className="squareBtn"><i className="fa fa-file-upload"></i></button>
                    </div>
                    <div className="spaceRight">
                        <button className="squareBtn"><i class="fa fa-download"></i></button>
                    </div>
                    <button className="squareBtn"><i class="fa fa-users"></i></button>
                </div>
                <div className="btnTable">
                    <div className="btnRow">
                        <Link to="/Roll">
                            <div className="spaceRight">
                                <button>A Star</button>
                            </div>
                        </Link>
                        <div className="spaceRight">
                            <button>B Star</button>
                        </div>
                        <button>C Star</button>
                    </div>

                    <div className="btnRow">
                        <div className="spaceRight">
                            <button>D Star</button>
                        </div>
                        <div className="spaceRight">
                            <button>E Star</button>
                        </div>
                        <button>F Star</button>
                    </div>

                    <div className="btnRow" style={{marginBottom: 5}}>
                        <div className="spaceRight">
                            <button>G Star</button>
                        </div>
                        <div className="spaceRight">
                            <button>H Star</button>
                        </div>
                        <button>I Star</button>
                    </div>
                </div>

                <div className="btnRow">
                    <Link to="/">
                        <button>Return</button>
                    </Link>
                </div>
            </div>
        )
    }
}

export default Choose;
