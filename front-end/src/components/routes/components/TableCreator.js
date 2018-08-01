import React from 'react';
import '../../../styles/App.css';

class TableCreator extends React.Component {
    render() {
        const addRow = this.props.addRow,
              removeRow = this.props.removeRow,
              saveValue = this.props.saveValue,
              saveRoll = this.props.saveRoll;
        let createRows = this.props.results.map(function(result, i) {
            return (
                <tr key={i}>
                    <td><input type="text" name="roll" value={result.roll} maxLength="10" onChange={(event) => saveRoll(i,event)}/></td>
                    <td className="leftAlign">
                        <input type="text" name="value" value={result.value} maxLength="32" onChange={(event) => saveValue(i, event)}/>
                    </td>
                    <td style={{borderRight:"none", borderTop:"1px solid white"}}>
                        <button type="button" className="tinyBtn squareBtn" onClick={(e) => {removeRow(i, e)}}>
                            <i className="fa fa-minus"></i>
                        </button>
                        <button type="button" className="tinyBtn squareBtn" onClick={(e) => {addRow(e)}}>
                            <i className="fa fa-plus"></i>
                        </button>
                    </td>
                </tr>
            )
        })

        return (
            <div>
                <form>
                    <h1><input type="text" name="title" value={this.props.title} maxLength="32" onChange={this.props.saveTable}/></h1>
                    <hr/>
                    <table>
                        <tbody>
                            <tr>
                                <th><input type="text" name="dice" value={this.props.dice} maxLength="10" onChange={this.props.saveTable}/></th>
                                <th className='leftAlign'>
                                    <input type="text" name="heading" value={this.props.heading} maxLength="32" onChange={this.props.saveTable}/>
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
