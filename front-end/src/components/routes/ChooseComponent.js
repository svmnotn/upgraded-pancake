import React from 'react';
import '../../styles/App.css';
import { Link } from 'react-router-dom';

import LinkBtn from './components/LinkBtn'
class Choose extends React.Component {
    render() {
        return(
            <div>
                <h1>Roll for: </h1>
                <hr/>
                <table className="btnTable">
                    <tbody>
                        <tr>
                            <td><button className="squareBtn"><i className="fa fa-file-upload"></i></button></td>
                            <td><button className="squareBtn"><i className="fa fa-download"></i></button></td>
                            <td><button className="squareBtn"><i className="fa fa-users"></i></button></td>
                        </tr>

                        <tr>
                            <td>
                                <LinkBtn btnName="A-Star" url="table/static"/>
                            </td>
                            <td>
                                <LinkBtn btnName="B-Star" url="table/static"/>
                            </td>
                            <td>
                                <LinkBtn btnName="C-Star" url="table/static"/>
                            </td>
                        </tr>

                        <tr>
                            <td>
                                <LinkBtn btnName="D-Star" url="table/static"/>
                            </td>
                            <td>
                                <LinkBtn btnName="E-Star" url="table/static"/>
                            </td>
                            <td>
                                <LinkBtn btnName="F-Star" url="table/static"/>
                            </td>
                        </tr>

                        <tr>
                            <td>
                                <LinkBtn btnName="G-Star" url="table/static"/>
                            </td>
                            <td>
                                <LinkBtn btnName="H-Star" url="table/static"/>
                            </td>
                            <td>
                                <LinkBtn btnName="I-Star" url="table"/>
                            </td>
                        </tr>

                        <tr>
                            <td></td>
                            <td>
                                <Link to="/"><button>Return</button></Link>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        )
    }
}

export default Choose;