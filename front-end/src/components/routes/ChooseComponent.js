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
        if (arr <= 0) {
            return (
                <td>
                    <LinkBtn btnName={arr} url= {"table/".concat(arr)} />
                </td>
            )
        } else {
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

        tempTables.map(function (table, i) {
            tempRow.push(table);
            if ((i + 1) % 3 === 0) {
                savedTables.push(tempRow);
                tempRow = [];
            } else if (i + 1 === tempTables.length) {
                savedTables.push(tempRow);
            }
        })

        return (
            savedTables.map(function (row, i) {
                return (
                    <tr key={i}>
                        {this.createRows(row)}
                    </tr>
                )
            }, this)
        );
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
                            <td className="rightAlign">
                                <button className="tinyButton">
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
