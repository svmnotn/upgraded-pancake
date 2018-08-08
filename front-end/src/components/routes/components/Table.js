import React from 'react';
import '../../../styles/App.css';
import axios from 'axios';
import {Link, withRouter } from 'react-router-dom';

class Table extends React.Component {
    constructor(props) {
        super(props);
        console.log(this.props.results);
        this.state = {
            tableResult: this.getResult(),
        }
        this.getResult = this.getResult.bind(this);
        this.createCols = this.createCols.bind(this);
    }

    getResult() {
        axios.post('/table', {
            heading: this.props.heading,
            dice: this.props.dice,
            results:  this.props.results,
        }).then ((response) => {
            this.setState({
                tableResult: response.data.value
            })
        }).catch(function(error){
            console.log(error);
        })
    }

    reroll() {
        this.getResult();
    }

    //Assumes that the value of results[key] is already an array of strings
    createCols (valArr) {
        return (
           valArr.map (function (result, i){
               return (
                   <td key={i} className="leftAlign">{result}</td>
               )
           })
        )
    }

    render () {
        let createRows = this.props.results.map(function(result, i) {
            if (typeof result.value === "string") {
                return (
                    <tr key={i}>
                        <td>{result.roll}</td>
                        <td className="leftAlign">{result.value}</td>
                    </tr>
                )
            }

            else {
                return (
                    <tr key={i}>
                        <td>{result.roll}</td>
                        {this.createCols(result.value)}
                    </tr>

                )
            }
        }, this)

        return (
            <div>
                <h1>{this.props.title}</h1>
                <hr/>
                <div className="tableAlign">
                    <table>
                        <tbody>
                            <tr>
                                <th>{this.props.dice}</th>
                                <th className='leftAlign'>{this.props.heading}</th>
                            </tr>
                            {createRows}
                        </tbody>
                    </table>
                </div>

                <p className="result">{this.state.tableResult}</p>

                <div className="btnRow">
                <table style={{border: "none"}}>
                    <tbody>
                        <tr>
                            <td style={{border: "none"}}>
                                <button style= {{marginRight: '5%',}}
                                        onClick={()=>{this.reroll()}}>Reroll</button>
                            </td>

                            <td style={{border: "none"}}>
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

                            <td style={{border: "none"}}>
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
