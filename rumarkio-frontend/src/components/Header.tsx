import HeaderTitle from '../molecules/HeaderTitle';
import {HeaderTitleProp} from '../interfaces/MoleProps';
import Link from 'next/link';
import HeaderLogo from './HeaderLogo';

export default function Header() {
  const header_title: HeaderTitleProp = {
    title: "$_>"
  };

  return (
    <header className="bg-primary-orange py-4">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex items-center justify-between">
        
        <div className="flex items-center">
          <Link href="/" passHref>
            <span className="flex items-center space-x-1 cursor-pointer">
              <HeaderTitle title={header_title.title} />
            </span>
          </Link>
        </div>
        <div className="flex items-center">
          <Link href="/" passHref>
            <span className="flex items-center space-x-1 cursor-pointer">
              <HeaderLogo />
            </span>
          </Link>
        </div>
        <div className="font-fira-mono text-2xl hidden md:block">
          <ul className="ml-4 flex items-center space-x-4">
            <li>
              <Link href="/about" passHref>
                <span className="text-gray-100 cursor-pointer">About</span>
              </Link>
            </li>
            <li>
              <Link href="/login" passHref>
                <span className="text-gray-100 cursor-pointer">Login</span>
              </Link>
            </li>
            <li>
              <Link href="/download" passHref>
                <span className="px-4 py-2 rounded-full font-medium cursor-pointer">
                  Download
                </span>
              </Link>
            </li>
          </ul>
        </div>
      </div>
    </header>
  );
}
