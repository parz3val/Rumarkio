// Basic Title 
import {TitleProps} from '../interfaces/AtomProps';

export default function Title(props: TitleProps) {
    return (
        <h1 className="font-medium text-2xl font-fira-code">{props.title}</h1>
    );
}
