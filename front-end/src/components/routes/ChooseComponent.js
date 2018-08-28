import React from 'react';
import '../../styles/App.css';

import { Link } from 'react-router-dom';
import LinkBtn from './components/LinkBtn'
class Choose extends React.Component {
    constructor(props) {
        super(props);
        this.createRows = this.createRows.bind(this);
        this.createTable = this.createTable.bind(this);
    }

    createRows(arr){
        console.log("CreateRows Received: " + arr);
        if (typeof arr !== "object") {
            return (
                <td>
                    <LinkBtn btnName={arr} url= {"table/".concat(arr)} />
                </td>
            )
        }

        else {
            return (
                arr.map(function(tableName, i) {
                    return (
                        <td key={i}>
                            <LinkBtn btnName={tableName} url= {"table/".concat(tableName)} />
                        </td>
                    )
                },this)
            )
        }
    }

    createTable() {
        let tempTables = this.props.location.state.savedTables,
            savedTables = [], tempRow = [];
        console.log("TempTables: " + tempTables);

        for (let i = 0; i < tempTables.length; i++) {
            if (tempTables.length === 1) {
                savedTables.push(tempTables[0]);
            }

            else {
                tempRow.push(tempTables[i]);
                console.log(tempRow);
                if (i + 1 > tempTables.length - 1 || (i % 2 === 0 && i !== 0)) {
                    savedTables.push(tempRow);
                    tempRow = [];
                }
            }
        }

        console.log("SavedTables2: " + savedTables);
        return (
        savedTables.map(function (row, i) {
            return (
                <tr key={i}>
                    {this.createRows(row)}
                </tr>
            )
        }, this));
       /* for (let i = 0; i < savedTables.length; i + 3) {
            if (i + 2 < savedTables.length + 1) {
                console.log("Table forms a row of 3 & has a remainder");
                return (
                    <tr>
                        {this.createRows(savedTables.slice(i, i + 3))}
                    </tr>
                )
            }

            else {
                console.log("Table ends here");
                return (
                    <tr>
                        {this.createRows(savedTables.slice(i, savedTables.length + 1))}
                    </tr>
                )
            }
        }*/
    }

    render() {
        return(
            <div>
                <h1>Roll for: </h1>
                <hr/>

                <table className="btnTable">
                    <tbody>
                        {this.createTable()}
                        <tr>
                            <td style={{textAlign: "right"}}>
                                <button style={{height: "8vh", width: "8vh", fontSize:"1em"}}>
                                    <i className="fa fa-file-upload"></i>
                                </button>
                            </td>
                            <td>
                                <Link to="/"><button>Return</button></Link>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        )
        //console.log(this.props.location.state.savedTables);
        /*return(
            <div>
                <h1>Roll for: </h1>
                <hr/>

                <table className="btnTable">
                    <tbody>
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
                                <LinkBtn btnName="I-Star" url="table/static"/>
                            </td>
                        </tr>

                        <tr>
                            <td style={{textAlign: "right"}}>
                                <button style={{height: "8vh", width: "8vh", fontSize:"1em"}}>
                                    <i className="fa fa-file-upload"></i>
                                </button>
                            </td>
                            <td>
                                <Link to="/"><button>Return</button></Link>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        )*/
    }
}

export default Choose;
