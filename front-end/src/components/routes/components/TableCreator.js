import React from 'react';
import '../../../styles/App.css';

class TableCreator extends React.Component {
    render() {
        const removeRow = this.props.removeRow,
              saveValue = this.props.saveValue,
              saveRoll = this.props.saveRoll;

        let createRows = this.props.results.map(function(result, i) {
            return (
                <tr key={i}>
                    <td style={{width:"20%"}}>
                        <input type="text"
                               name="roll"
                               value={result.roll}
                               maxLength="6"
                               onChange={(event) => saveRoll(i,event)}/>
                    </td>

                    <td style={{width:"80%"}}>
                        <input type="text"
                               name="value"
                               value={result.value}
                               maxLength="55"
                               onChange={(event) => saveValue(i, event)}
                               className="leftAlign"/>
                    </td>

                    <td style={{borderRight:"none", borderTop:"1px solid white"}}>
                        <button type="button"
                                className="tinyBtn squareBtn"
                                onClick={(e) => {removeRow(i, e)}}>
                            <i className="fa fa-minus"></i>
                        </button>
                    </td>
                </tr>
            )
        })

        return (
            <div>
                <form>
                    <h1>
                        <input type="text"
                               name="title"
                               value={this.props.title}
                               maxLength="21"
                               onChange={this.props.saveTable}
                               className="h1Input"/>
                    </h1>
                    <hr/>
                    <table>
                        <tbody>
                            <tr>
                                <th style={{width:"20%"}}>
                                    <input type="text"
                                           name="dice"
                                           value={this.props.dice}
                                           maxLength="6"
                                           onChange={this.props.saveTable}
                                           className="thInput"/>
                                </th>

                                <th style={{width:"80%"}}>
                                    <input type="text"
                                           name="heading"
                                           value={this.props.heading}
                                           maxLength="55"
                                           onChange={this.props.saveTable}
                                           className="leftAlign thInput"/>
                                </th>
                            </tr>
                            {createRows}
                        </tbody>
                    </table>
                </form>
            </div>
        )
    }
}

export default TableCreator;
