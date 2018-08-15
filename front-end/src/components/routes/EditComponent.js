import React from 'react';
import '../../styles/App.css';

import TableEditor from './components/TableEditor';

class Edit extends React.Component {

    /*Checks if the user is editing a pre-existing table or a completely new table*/
    editTable() {
        if(this.props.location.state) {
            return (
                <TableEditor title={this.props.location.state.title}
                             heading={this.props.location.state.heading}
                             dice = {this.props.location.state.dice}
                             results = {this.props.location.state.results}/>
            )
        } else {
            return (
                <TableEditor/>
            )
        }
    }

    render() {
        return (
            <div>
                {this.editTable()}
            </div>
        )
    }
}

export default Edit;
