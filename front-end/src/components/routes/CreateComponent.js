import React from 'react';
import '../../styles/App.css';
import { Link } from 'react-router-dom';

import TableCreator from './components/TableCreator';
class Create extends React.Component {
    constructor(){
        super();
        this.state = {
            hasSaved: false,
            title: "Default Table",
            heading: "Categories",
            dice: "1d2",
            results: [{roll: 1, value:"Data",}, {roll:2, value: "Data1"}],
        }

        this.addRow = this.addRow.bind(this);
        this.removeRow = this.removeRow.bind(this);
        this.saveTable = this.saveTable.bind(this);
        this.saveValue = this.saveValue.bind(this);
        this.saveRoll = this.saveRoll.bind(this);
    }

    addRow(e) {
        this.setState((prevState) => {
            return {
                results: prevState.results.concat({roll: 1, value: "Default Text"}),
            }
        })
        e.preventDefault();
    }

   removeRow(key,e) {
       console.log(key);
        this.setState(function (prevState) {
            return {
                results: prevState.results.filter(function(result, i){
                    console.log("This is: "+ i);
                    return i !== key;
                })
            }
        })
        e.preventDefault();
    }

    /*saveTitle(event) {
        this.setState ({
            title:event.target.value,
        });
    }*/
    /*handleChange = (e) => {
    const items = this.state.items;
    items[1].role = e.target.value;

    // re-render
    this.forceUpdate();
};*/
  /*result[key].value = event.target.value;
        console.log("This is the value of i: " + key + " , value of change: " + result[key].value);*/
    saveValue(key, event) {
        const results = this.state.results;
        results[key].value = event.target.value;

        this.forceUpdate();
    }

    saveRoll(key, event) {
        const results = this.state.results;
        results[key].roll = parseInt(event.target.value, 10);

        this.forceUpdate();
    }

    saveTable (event) {
        console.log(event.target.name);
        this.setState({
            [event.target.name]: event.target.value,
        })
    }

    render() {
        return(
            <div>
                <TableCreator title={this.state.title}
                              heading={this.state.heading}
                              dice={this.state.dice}
                              results={this.state.results}
                              addRow={this.addRow}
                              removeRow={this.removeRow}
                              saveTable={this.saveTable}
                              saveValue={this.saveValue}
                              saveRoll={this.saveRoll}/>

                <div className='container'>
                    <Link to="/">
                        <button className="squareBtn"><i className="fa fa-times"></i></button>
                    </Link>
                    <button className="squareBtn"><i className="fa fa-save"></i></button>
                    <Link to={{ pathname: "/roll",
                                state: { title: this.state.title,
                                         heading: this.state.heading,
                                         dice: this.state.dice,
                                         results: this.state.results,
                    }}}>
                        <button className="squareBtn"><i className="fa fa-check"></i></button>
                    </Link>
                </div>
            </div>
        )
    }
}
//<div style={{marginLeft: '30%', marginRight: '45%'}}></div>
export default Create;
