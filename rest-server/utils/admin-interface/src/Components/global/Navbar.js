import React from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCircleInfo, faFileLines, faRightFromBracket, faTerminal } from '@fortawesome/free-solid-svg-icons';
import styles from './navbar.module.css';
import Banner from '../../svgs/Axlyn.svg'

const Navbar = (props) => {
  return (
    <nav>
      <div className={styles.navbar}>
        <div className={styles.navbar_left}>
          <a className={styles.logo} href="/">
            <img src={Banner} alt="axlyn" width="90px" />
          </a>
        </div>
        <div className={styles.navbar_right}>
          <h3 className={styles.version}>Version | 0.1.1</h3>
        </div>
      </div>
      <div className={styles.navbar}>
        <div className={styles.navbar_left}>
          <a 
            className={`${styles.navLinks} ${props.active === "dashboard" && styles.navLinks_selected}`} 
            onClick={()=>props.onSelect("dashboard")}
            href="#Dashboard">Dashboard</a>
          <a 
            className={`${styles.navLinks} ${props.active === "database" && styles.navLinks_selected}`} 
            onClick={()=>props.onSelect("database")}
            href="#Database">Database</a>
          <a 
            className={`${styles.navLinks} ${props.active === "endpoints" && styles.navLinks_selected}`} 
            onClick={()=>props.onSelect("endpoints")}
            href="#Endpoints">Endpoints</a>
          <a 
            className={`${styles.navLinks} ${props.active === "backup" && styles.navLinks_selected}`} 
            onClick={()=>props.onSelect("backup")}
            href="#Backup">Backup</a>
          <a 
            className={`${styles.navLinks} ${props.active === "users" && styles.navLinks_selected}`} 
            onClick={()=>props.onSelect("users")}
            href="#Users">Users</a>
          <a 
            className={`${styles.navLinks} ${props.active === "settings" && styles.navLinks_selected}`} 
            onClick={()=>props.onSelect("settings")}
            href="#Settings">Settings</a>
          <a 
            className={`${styles.navLinks} ${props.active === "plugin" && styles.navLinks_selected}`} 
            onClick={()=>props.onSelect("plugin")}
            href="#Plugin">Plugin</a>
        </div>
        <div className={styles.navbar_right}>
          <FontAwesomeIcon className={styles.icon} icon={faRightFromBracket} />
          <FontAwesomeIcon className={styles.icon} icon={faTerminal} />
          <FontAwesomeIcon className={styles.icon} icon={faCircleInfo} />
          <FontAwesomeIcon className={styles.icon} icon={faFileLines} />
        </div>
      </div>
    </nav>
  )
}

export default Navbar