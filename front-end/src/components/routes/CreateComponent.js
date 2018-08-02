import React from 'react';
import '../../styles/App.css';
import { Link } from 'react-router-dom';

import TableCreator from './components/TableCreator';
class Create extends React.Component {
    constructor(){
        super();
        this.state = {
            title: "Default Table",
            heading: "Categories",
            dice: "1d2",
            results: [{roll: 1, value:"Data",}, {roll:2, value: "Data1"}],
            passedTest: false,
        }

        this.addRow = this.addRow.bind(this);
        this.removeRow = this.removeRow.bind(this);
        this.saveTable = this.saveTable.bind(this);
        this.saveValue = this.saveValue.bind(this);
        this.saveRoll = this.saveRoll.bind(this);
        this.checkTable = this.checkTable.bind(this);
        this.updateTable = this.updateTable.bind(this);
    }

    updateTable(title, heading, dice, results) {
        this.setState ({
            title: title,
            heading: heading,
            dice: dice,
            results: results,
        })
        console.log("I have worked!");
    }

    addRow(event) {
        this.setState((prevState) => {
            return {
                results: prevState.results.concat({roll: 1, value: "Default Text"}),
            }
        })

        this.setState({
            passedTest: false,
        })
        event.preventDefault();
    }

   removeRow(key,event) {
        this.setState(function (prevState) {
            return {
                results: prevState.results.filter(function(result, i){
                    return i !== key;
                })
            }
        })

        this.setState({
            passedTest: false,
        })
        event.preventDefault();
    }

    saveValue(key, event) {
        const results = this.state.results;
        results[key].value = event.target.value;
        this.setState({
            passedTest: false,
        })
        this.forceUpdate();
    }

    saveRoll(key, event) {
        const results = this.state.results;
        if(event.target.value.trim() === "") {
            results[key].roll = "";
        }

        else {
            const alphaRegEx = /[A-Z][a-z]/,
                  tempArr = event.target.value.split("-");

            if(alphaRegEx.test(event.target.value)) {
                return;
            }

            else {
                if(tempArr > 1 || event.target.value.includes("-")) {
                    results[key].roll = event.target.value;
                }

                else {
                    results[key].roll = parseInt(event.target.value, 10);
                }
            }
        }
        this.setState({
            passedTest: false,
        })
        this.forceUpdate();
    }

    saveTable (event) {
        this.setState({
            [event.target.name]: event.target.value,
            passedTest: false,
        })
    }

    checkTable() {
        const diceRegEx = /^\d*d\d+$/;
        if(diceRegEx.test(this.state.dice)) {
            let diceVal = this.state.dice.split("d");

            if(diceVal[0].length > 1) {
                diceVal = diceVal[0] * diceVal[1];
            }

            else {
                diceVal = diceVal[1];
            }

            for(let i = 0; i < this.state.results.length; i++) {
                if(this.state.results[i].roll > diceVal || this.state.results[i].roll <= 0) {
                    return;
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
                <button onClick={() => this.checkTable()} className="squareBtn"><i className="fa fa-check"></i></button>
            )
        }
    }

    render() {
        return(
            <div>
                <TableCreator title={this.state.title}
                              heading={this.state.heading}
                              dice={this.state.dice}
                              results={this.state.results}
                              removeRow={this.removeRow}
                              saveTable={this.saveTable}
                              saveValue={this.saveValue}
                              saveRoll={this.saveRoll}/>

                <div className='container'>
                    <Link to="/">
                        <button className="squareBtn"><i className="fa fa-times"></i></button>
                    </Link>
                    <button type="button"
                            className="squareBtn"
                            onClick={(event) => this.addRow(event)}>
                        <i className="fa fa-plus"></i>
                    </button>
                    {this.confirmTable()}
                </div>
            </div>
        )
    }
}

export default Create;
