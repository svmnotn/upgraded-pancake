import React from 'react';
import '../../../styles/App.css';
import axios from 'axios';
import { Link } from 'react-router-dom';

class TableEditor extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            title: "Default Table",
            heading: "Category",
            dice: "1d2",
            results: [{roll: 1, value:"Some Data"}, {roll:2, value:"Some More Data"}],
            passedTest: false,
        }

        this.addRow = this.addRow.bind(this);
        this.removeRow = this.removeRow.bind(this);
        this.saveProp = this.saveProp.bind(this);
        this.saveValue = this.saveValue.bind(this);
        this.saveRoll = this.saveRoll.bind(this);
        this.testTable = this.testTable.bind(this);
        this.createRows = this.createRows.bind(this);
    }

    // If the user wanted to edit a preexisting table, update the table's values to the prexisting one.
    componentDidMount() {
        if(this.props.title) {
            this.setState({
                title: this.props.title,
                heading: this.props.heading,
                dice: this.props.dice,
                results: this.props.results,
            })
        }
    }

    // Add a new row to the table by adding a new object into the results.values array
    addRow(event) {
        let tempCols;

        if (typeof this.state.heading !== "string") {
            tempCols = [];

            this.state.heading.forEach(function () {
                tempCols.push("Default");
            });
        } else {
            tempCols = "Default";
        }

        const newRow = {roll: 1, value: tempCols}

        this.setState((prevState) => {
            return {
                results: prevState.results.concat(newRow),
            }
        })

        this.setState({
            passedTest: false,
        })

        event.preventDefault();
    }

    // Add a new column to the table by checking whether or not if the value is a string/array and adjusting accordingly
    addCol(event) {
        event.preventDefault();
        let results = this.state.results;
        let header = this.state.heading;

        results.forEach (function (result) {
            if(typeof result.value !== "string") {
                result.value.push("Default");
            } else {
                result.value = [result.value, "Default"];
            }
        });

        if (typeof header !== "string") {
            header.push("Category");
        } else {
            header = [header, "Cate1"];
        }

        this.setState({
            heading: header,
            results: results,
            passedTest: false,
        })
    }

    //Removes a column from the table by checking whether or not if the value is a string/array and adjusting accordingly
    removeCol(event) {
        event.preventDefault();
        let results = this.state.results;

        results.forEach(function (result) {
            if(typeof result.value !== "string") {
                if (result.value.length === 1) {
                    // TODO Better error
                    // This should **NOT** happen. Make sure of it.
                    console.log("ERROR: THIS SHOULD BE A STRING ALREADY!");
                }

                result.value.pop();

                if (result.value.length === 1) {
                    // If there is only one value,
                    // convert it to a single string
                    result.value = result.value[0];
                }
            } else {
                // Clear the last column
                result.value = "";
            }
        });

        let header = this.state.heading;

        if (typeof header !== "string") {
            if (header.length === 1) {
                // This should **NOT** happen. Make sure of it.
                console.log("ERROR: THIS SHOULD BE A STRING ALREADY!");
            }

            header.pop();

            if (header.length === 1) {
                // If there is only one value, convert
                // it to a single string
                header = header[0];
            }

            this.setState({
                heading: header,
                results: results,
                passedTest: false,
            });
        } else {
            // Clear the last column
            this.setState({
                heading: "",
                results: results,
                passedTest: false,
            });
        }
    }

    //Removes a row from the table by checking whether the key matches an element in the array
    removeRow(key, event) {
        event.preventDefault();
        this.setState(function (prevState) {
            return {
                results: prevState.results.filter(function (result, i) {
                    return i !== key;
                })
            }
        })

        this.setState({
            passedTest: false,
        });
    }

    //Saves the user's typed value to the correct text input
    saveValue(resKey, valKey, event) {
        const results = this.state.results[resKey];

        if (typeof results.value === "string") {
            results.value = event.target.value;
        } else {
            results.value[valKey] = event.target.value;
        }

        this.setState ({
            passedTest: false,
        });
        this.forceUpdate();
    }

    //Saves the user's typed value to the correct text input. User is limited to numerical characters and the letter d.
    saveRoll(key, event) {
        const results = this.state.results;

        if(event.target.value.trim() === "") {
            results[key].roll = "";
        } else {
            const alphaRegEx = /[A-Z][a-z]/,
                  tempArr = event.target.value.split("-");

            if(alphaRegEx.test(event.target.value)) {
                return;
            } else if(tempArr > 1 || event.target.value.includes("-")) {
                results[key].roll = event.target.value;
            } else {
                results[key].roll = parseInt(event.target.value, 10);
            }
        }

        this.setState({
            passedTest: false,
        });

        this.forceUpdate();
    }

    //Saves the user's typed value to the correct text input. Used for table name and dice.
    saveProp (event) {
        this.setState ({
            [event.target.name]: event.target.value,
            passedTest: false,
        })
    }

    //Tests if the table is in valid dice notation and it does not break any set logic
    testTable () {
        axios.post ('/table/validate', {
            heading: this.state.heading,
            dice: this.state.dice,
            results:  this.state.results,
        }).then ((response) => {
            console.log(response);
            if (response.data) {
                this.setState({
                    passedTest: true,
                })
            }
        }).catch (function (error) {
            console.log(error);
        }, this)
    }

    //Forces the user to check their table before being able to roll on it. If a change it made on the table, the user is forced to recheck their table.
    confirmTable() {
        if(this.state.passedTest) {
            return (
                <Link to = {{ pathname: "/roll",
                              state: { title: this.state.title,
                                       heading: this.state.heading,
                                       dice: this.state.dice,
                                       results: this.state.results,
                }}}>
                    <button className="squareBtn"><i className="fa fa-arrow-right"></i></button>
                </Link>
            )
        }

        else {
            return (
                <button onClick={() => this.testTable()} className="squareBtn"><i className="fa fa-check"></i></button>
            )
        }
    }

    createRows() {
        return (
            this.state.results.map(function(result, i) {
                return (
                    <tr key={i}>
                        <td style={{width:"20%"}}>
                            <input type="text"
                                   name="roll"
                                   value={result.roll}
                                   maxLength="6"
                                   onChange={(event) => this.saveRoll(i,event)}/>
                        </td>

                        {this.createCols(i, result.value)}

                        <td style={{borderRight:"none", borderTop:"1px solid white"}}>
                            <button type="button"
                                    className="tinyBtn squareBtn"
                                    onClick={(e) => {this.removeRow(i, e)}}>
                                <i className="fa fa-minus"></i>
                            </button>
                        </td>
                    </tr>
                )
            }, this)
        )
    }

    //Checks whether the value is a string or an array before returning
    createCols(resKey, val) {
        if(typeof val === "string") {
            return (
                <td>
                    <input type="text"
                           value={val}
                           maxLength="55"
                           onChange={(event) => {this.saveValue(resKey, 0, event)}}
                           className="leftAlign"/>
                </td>
            )
        } else {
            return (
                val.map (function (value, i) {
                    return (
                        <td key={i}>
                            <input type="text"
                                   placeholder={value}
                                   maxLength="55"
                                   className="leftAlign"
                                   onChange = {(event) => this.saveValue (resKey, i, event)}/>
                        </td>
                    )
                },this)
            )
        }
    }

    createHeading(header) {
        if(typeof header === "string") {
            return (
                <th>
                    <input type="text"
                            name="heading"
                            value={header}
                            onChange={this.saveProp}
                            maxLength="55"
                            className="leftAlign thInput"/>
                </th>
            )
        } else {
            return (
                header.map(function (heading, i) {
                    return (
                        <th>
                            <input type="text"
                                   name="heading"
                                   placeholder = {heading}
                                   maxLength="55"
                                   className="leftAlign thInput"/>
                        </th>
                    )
                })
            )
        }
    }

    createEmptyCols() {
        if(typeof this.state.heading !== "string") {
            let tempVal = [];

            this.state.heading.forEach (function () {
                tempVal.push(<td></td>);
            });

            return (
                <tr>
                    {tempVal}
                    <td></td>
                    <td style= {{borderRight: "none"}}><button className="tinyBtn squareBtn"
                        onClick={(event) => this.addRow( event)}>
                            <i className="fa fa-plus"></i>
                        </button>
                    </td>
                </tr>
            )
        } else {
            return (
                <tr>
                    <td> </td>
                    <td> </td>
                    <td>
                        <button className="tinyBtn squareBtn"
                                onClick={(event) => this.addRow( event)}>
                            <i className="fa fa-plus"></i>
                        </button>
                    </td>
                </tr>
            )
        }
    }

    render () {
        return (
            <div>
                <div>
                    <form>
                        <h1>
                            <input type="text"
                                   name="title"
                                   value={this.state.title}
                                   onChange={this.saveProp}
                                   maxLength="21"
                                   className="h1Input"/>
                        </h1>
                        <hr/>
                        <table>
                            <tbody>
                                <tr>
                                    <th style={{width:"20%"}}>
                                        <input type="text"
                                               name="dice"
                                               value={this.state.dice}
                                               onChange={this.saveProp}
                                               maxLength="6"
                                               className="thInput"/>
                                    </th>

                                    {this.createHeading(this.state.heading)}

                                    <td style={{borderRight:"none"}}>
                                        <button style={{marginTop: "0.7em"}}
                                                className="tinyBtn squareBtn"
                                                onClick={(event) => this.removeCol(event)}>
                                            <i className="fa fa-minus"></i>
                                        </button>
                                        <button style={{marginTop: "0.7em"}}
                                                className="tinyBtn squareBtn"
                                                onClick={(event) => this.addCol(event)}>
                                            <i className="fa fa-plus"></i>
                                        </button>
                                    </td>
                                </tr>

                                {this.createRows()}

                                {this.createEmptyCols()}
                            </tbody>
                        </table>
                    </form>
                </div>

                <div className="container">
                    <Link to="/">
                        <button className="squareBtn"><i className="fa fa-times"></i></button>
                    </Link>

                    {this.confirmTable()}
                </div>
            </div>
        )
    }
}

export default TableEditor;
