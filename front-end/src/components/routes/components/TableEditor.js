import React from 'react';
import '../../../styles/App.css';
import { Link } from 'react-router-dom';

class TableEditor extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            title: "Default Table",
            heading: ["Cate1", "Cate2"],
            dice: "1d2",
            results: [{roll: 1, value:["Data1", "New Data"]}, {roll:2, value:["Data1", "New Data"]}],
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

    //If something was received by the component, update component values.
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

    addRow(event) {
        let tempCols = [];

        for (let i = 0; i < this.state.heading.length; i++) {
            tempCols.push("Default");
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

    addCol(event) {
        let tempResult = this.state.results,
            tempHeader = this.state.heading;

        this.state.results.map (function (result, i) {
            if(typeof result.value !== "string") {
                result.value.push("Default");
            }
            else {
                result.value = [result.value, "Default"];
            }
        }, this)

        if (typeof tempHeader === "string") {
            tempHeader = [tempHeader, "Category1"];
        }

        else {
            tempHeader.push("Category");
        }

        this.setState({
            heading: tempHeader,
            results: tempResult,
            passedTest: false,
        })
        event.preventDefault();
    }

    removeCol(event) {
        event.preventDefault();
        let tempResults = this.state.results;

        tempResults.map(function (result, i) {
            if(typeof result.value !== "string") {
                result.value.pop();
            } else {
                result.value = "";
            }
        }, this)

        if (typeof this.state.heading !== "string") {
            this.state.heading.pop();
            this.setState({
                results: tempResults,
                passedTest: false,
            });
        } else {
            this.setState({
                heading: "",
                results: tempResults,
                passedTest: false,
            });
        }
    }

    removeRow(key, event) {
        this.setState(function (prevState) {
            return {
                results: prevState.results.filter(function (result, i) {
                    return i !== key;
                })
            }
        })

        this.setState({
            passedTest: false,
        })
        event.preventDefault();
    }

    saveValue(resKey, valKey, event) {
        const results = this.state.results[resKey];

        if (typeof results.value === "string") {
            results.value = event.target.value;
        } else {
            results.value[valKey] = event.target.value;
        }

        this.setState ({
            passedTest: false,
        })
        this.forceUpdate();
    }

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
        })

        this.forceUpdate();
    }

    saveProp (event) {
        this.setState ({
            [event.target.name]: event.target.value,
            passedTest: false,
        })
    }

    testTable() {
        const diceRegEx = /^\d*d\d+$/;
        if(diceRegEx.test(this.state.dice)) {
            let diceVal = this.state.dice.split("d");

            if(diceVal[0].length > 1) {
                diceVal = diceVal[0] * diceVal[1];
            } else {
                diceVal = diceVal[1];
            }

            for (let roll in this.state.results) {
                if(typeof roll === "string") {
                    let tempVal = roll.split("-");
                    if((tempVal[0] > diceVal && tempVal[1] > diceVal) || (tempVal[0] <= 0 && tempVal[1] <= 0)) {
                        return;
                    }
                } else if(roll > diceVal || roll <= 0) {
                    return
                }
            }

            this.setState({
                passedTest: true,
            })
        }
    }

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

            this.state.heading.map (function (header, i) {
                tempVal.push(<td></td>);
            })

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
