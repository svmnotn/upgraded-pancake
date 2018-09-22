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

        for (let i = 0; i < tempTables.length; i++) {
            if (tempTables.length === 1) {
                savedTables.push(tempTables[0]);
            }

            else {
                tempRow.push(tempTables[i]);
                if (i + 1 > tempTables.length - 1 || (i % 2 === 0 && i !== 0)) {
                    savedTables.push(tempRow);
                    tempRow = [];
                }
            }
        }

        return (
        savedTables.map(function (row, i) {
            return (
                <tr key={i}>
                    {this.createRows(row)}
                </tr>
            )
        }, this));
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
    }
}

export default Choose;
