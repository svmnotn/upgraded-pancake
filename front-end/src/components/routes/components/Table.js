import React from 'react';
import '../../../styles/App.css';

import axios from 'axios';
import {Link, withRouter } from 'react-router-dom';

class Table extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            tableResult: this.getResult(),
            confirmTable: false,
            confirmText: "Sure?"
        }

        this.getResult = this.getResult.bind(this);
        this.createCols = this.createCols.bind(this);
        this.changeBool = this.changeBool.bind(this);
    }

    //Gets a random result on the table from the server and displays it
    getResult() {
        axios.post('/table', {
            heading: this.props.heading,
            dice: this.props.dice,
            results:  this.props.results,
        }).then ((response) => {
            let str = this.props.results[response.data.row].value;

            if (typeof this.props.results[response.data.row].value === "object") {
                str = this.props.results[response.data.row].value.join(", ");
            }

            this.setState({
                tableResult: <div>
                                 <b>Roll: </b>{response.data.roll},
                                 <b><br/>Value: </b>{str}
                             </div>
            })
        }).catch(function(error){
            console.log(error);
        }, this)
    }


    deleteTable() {
        let tempStr = "/table/".concat(this.props.title);
        this.setState ({
            confirmText: "Deleted!"
        })
        axios.delete (tempStr);
    }

    deleteConfirm() {
        if (this.state.confirmTable) {
            return (
                <button onClick={()=> {this.deleteTable()}}>{this.state.confirmText}</button>
            )
        } else {
            return (
                <button onClick={()=> {this.changeBool()}}>Delete</button>
            )
        }
    }

    changeBool() {
        if (this.state.confirmTable) {
            this.setState({
                confirmTable: false
            })
        } else {
            this.setState({
                confirmTable: true
            })
        }
    }

    //Allows the user to get another result if they do not like the one they received
    reroll() {
        this.getResult();
    }

    //Adds columns to create a multi-column table under the assumption that the value of results[key] is already an array of strings.
    createCols (valArr) {
        return (
           valArr.map (function (result, i) {
                return (
                    <td key={i} className="leftAlign">{result}</td>
                )
            })
        )
    }

    render () {
        //Creates rows and displays them differently depending on the number of columns
        let createRows = this.props.results.map(function(result, i) {
            if (typeof result.value === "string") {
                return (
                    <tr key={i}>
                        <td>{result.roll}</td>
                        <td className="leftAlign">{result.value}</td>
                    </tr>
                )
            } else {
                return (
                    <tr key={i}>
                        <td>{result.roll}</td>
                        {this.createCols(result.value)}
                    </tr>
                )
            }
        }, this)

        //Creates the columns for the heading depenind on the table's number of columns
        let createHeading = (heading) => {
            if (typeof heading === "string") {
                return (
                    <th className="leftAlign">{heading}</th>
                )
            } else {
                return(
                    heading.map (function (result, i) {
                        return (
                            <th key={i} className="leftAlign">{result}</th>
                        )
                    }
                ))
            }
        }

        return (
            <div>
                <h1>{this.props.title}</h1>
                <hr/>
                <div>
                    <table>
                        <tbody>
                            <tr>
                                <th>{this.props.dice}</th>
                                {createHeading(this.props.heading)}
                            </tr>
                            {createRows}
                        </tbody>
                    </table>
                </div>

                <p className="result">{this.state.tableResult}</p>

                <div className="btnRow">
                    <table>
                        <tbody>
                            <tr>
                                <td className="removeBrdr">
                                    <button className="marginRight"
                                            onClick={()=>{this.reroll()}}>Reroll</button>
                                </td>

                                <td className="removeBrdr">
                                    {this.deleteConfirm()}
                                </td>

                                <td className="removeBrdr">
                                    <Link to = {{ pathname: "/edit",
                                                state: { title: this.props.title,
                                                        heading: this.props.heading,
                                                        dice: this.props.dice,
                                                        results: this.props.results
                                                        }
                                                }}>
                                        <button> Edit </button>
                                    </Link>
                                </td>

                                <td className="removeBrdr">
                                    <Link to="/">
                                        <button>Return</button>
                                    </Link>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        )
    }
}

export default withRouter(Table);
