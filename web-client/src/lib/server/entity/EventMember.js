export default function (sequelize, DataTypes) {
	return sequelize.define(
		'EventMember',
		{
			event_id: { type: DataTypes.BIGINT, allowNull: false },
			user_id: { type: DataTypes.BIGINT, allowNull: false }
		},
		{
			sequelize,
			tableName: 'event_member',
			timestamps: false
		}
	);
}
